use crate::internal::Memory::*;
use crate::Common::*;
use crate::Math::Vec3;
use crate::Math::Vec4;
use crate::Math::{IVec2, IVec3, IVec4, Vec2};
use crate::Matrix::*;
use crate::Profiler::*;
use crate::Resource::*;
use crate::ResourceType::*;
use crate::ShaderState::*;
use crate::ShaderVar::*;
use crate::ShaderVarType::*;
use crate::StrMap::*;
use crate::Tex1D::*;
use crate::Tex2D::*;
use crate::Tex3D::*;
use crate::TexCube::*;
use crate::GL::gl;
use libc;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Shader {
    pub _refCount: u32,
    pub name: *const libc::c_char,
    pub vs: u32,
    pub fs: u32,
    pub program: u32,
    pub texIndex: u32,
    pub vars_size: i32,
    pub vars_capacity: i32,
    pub vars_data: *mut ShaderVar,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ShaderVar {
    pub type_0: ShaderVarType,
    pub name: *const libc::c_char,
    pub index: i32,
}

static mut includePath: *const libc::c_char = b"include/\0" as *const u8 as *const libc::c_char;

static mut versionString: *const libc::c_char =
    b"#version 120\n#define texture2DLod texture2D\n#define textureCubeLod textureCube\n\0"
        as *const u8 as *const libc::c_char;

static mut current: *mut Shader = std::ptr::null_mut();

static mut cache: *mut StrMap = std::ptr::null_mut();

unsafe extern "C" fn GetUniformIndex(mut this: *mut Shader, mut name: *const libc::c_char) -> i32 {
    if this.is_null() {
        Fatal(b"GetUniformIndex: No shader is bound\0" as *const u8 as *const libc::c_char);
    }
    let mut index: i32 = gl::GetUniformLocation((*this).program, name);
    index
}

unsafe extern "C" fn CreateGLShader(
    mut src: *const libc::c_char,
    mut type_0: gl::types::GLenum,
) -> u32 {
    let mut this: u32 = gl::CreateShader(type_0);

    let mut srcs: [*const libc::c_char; 2] = [versionString, src];

    gl::ShaderSource(
        this,
        2_i32,
        srcs.as_mut_ptr() as *const *const gl::types::GLchar,
        std::ptr::null(),
    );
    gl::CompileShader(this);

    /* Check for compile errors. */
    let mut status: i32 = 0;
    gl::GetShaderiv(this, gl::COMPILE_STATUS, &mut status);
    if status == 0 {
        let mut length: i32 = 0;
        gl::GetShaderiv(this, gl::INFO_LOG_LENGTH, &mut length);
        let mut infoLog: *mut libc::c_char =
            MemAllocZero((length + 1_i32) as usize) as *mut libc::c_char;
        gl::GetShaderInfoLog(this, length, std::ptr::null_mut(), infoLog);
        Fatal(
            b"CreateGLShader: Failed to compile shader:\n%s\0" as *const u8 as *const libc::c_char,
            infoLog,
        );
    }
    this
}

unsafe extern "C" fn CreateGLProgram(mut vs: u32, mut fs: u32) -> u32 {
    let mut this: u32 = gl::CreateProgram();
    gl::AttachShader(this, vs);
    gl::AttachShader(this, fs);

    /* TODO : Replace with custom directives. */
    gl::BindAttribLocation(
        this,
        0,
        b"vertex_position\0" as *const u8 as *const libc::c_char,
    );
    gl::BindAttribLocation(
        this,
        1_u32,
        b"vertex_normal\0" as *const u8 as *const libc::c_char,
    );
    gl::BindAttribLocation(
        this,
        2_u32,
        b"vertex_uv\0" as *const u8 as *const libc::c_char,
    );

    gl::LinkProgram(this);

    /* Check for link errors. */
    let mut status: i32 = 0;
    gl::GetProgramiv(this, gl::LINK_STATUS, &mut status);
    if status == 0_i32 {
        let mut length: i32 = 0;
        gl::GetProgramiv(this, gl::INFO_LOG_LENGTH, &mut length);
        let mut infoLog: *mut libc::c_char =
            MemAllocZero((length + 1_i32) as usize) as *mut libc::c_char;
        gl::GetProgramInfoLog(this, length, std::ptr::null_mut(), infoLog);
        Fatal(
            b"CreateGLProgram: Failed to link program:\n%s\0" as *const u8 as *const libc::c_char,
            infoLog,
        );
    }
    this
}

/* BUG : Cache does not contain information about custom preprocessor
 *       directives, hence cached shaders with custom directives do not work */
unsafe extern "C" fn GLSL_Load(
    mut name: *const libc::c_char,
    mut this: *mut Shader,
) -> *const libc::c_char {
    if cache.is_null() {
        cache = StrMap_Create(16_u32);
    }
    let mut cached: *mut libc::c_void = StrMap_Get(cache, name);
    if !cached.is_null() {
        return cached as *const libc::c_char;
    }
    let mut rawCode: *const libc::c_char = Resource_LoadCstr(ResourceType_Shader, name);
    let mut code: *const libc::c_char = StrReplace(
        rawCode,
        b"\r\n\0" as *const u8 as *const libc::c_char,
        b"\n\0" as *const u8 as *const libc::c_char,
    );
    StrFree(rawCode);
    code = GLSL_Preprocess(code, this);
    /* BUG : Disable GLSL caching until preprocessor cache works. */
    // StrMap_Set(cache, name, (void*)code);
    code
}

unsafe extern "C" fn GLSL_Preprocess(
    mut code: *const libc::c_char,
    mut this: *mut Shader,
) -> *const libc::c_char {
    let lenInclude: i32 = StrLen(b"#include\0" as *const u8 as *const libc::c_char) as i32;
    let mut begin: *const libc::c_char = std::ptr::null();

    /* Parse Includes. */
    loop {
        begin = StrFind(code, b"#include\0" as *const u8 as *const libc::c_char);
        if begin.is_null() {
            break;
        }
        let mut end: *const libc::c_char =
            StrFind(begin, b"\n\0" as *const u8 as *const libc::c_char);
        let mut name: *const libc::c_char =
            StrSubStr(begin.offset(lenInclude as isize).offset(1), end);
        let mut path: *const libc::c_char = StrAdd(includePath, name);
        let mut prev: *const libc::c_char = code;
        code = StrSub(code, begin, end, GLSL_Load(path, this));
        StrFree(prev);
        StrFree(path);
        StrFree(name);
    }

    /* Parse automatic ShaderVar stack bindings. */
    loop {
        begin = StrFind(code, b"#autovar\0" as *const u8 as *const libc::c_char);
        if begin.is_null() {
            break;
        }
        let mut end_0: *const libc::c_char =
            StrFind(begin, b"\n\0" as *const u8 as *const libc::c_char);
        let mut line: *const libc::c_char = StrSubStr(begin, end_0);
        let mut varType: [libc::c_char; 32] = [
            0_i32 as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let mut varName: [libc::c_char; 32] = [
            0_i32 as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        if libc::sscanf(
            line,
            b"#autovar %31s %31s\0" as *const u8 as *const libc::c_char,
            varType.as_mut_ptr(),
            varName.as_mut_ptr(),
        ) == 2_i32
        {
            let mut var: ShaderVar = ShaderVar {
                type_0: 0_i32,
                name: std::ptr::null(),
                index: 0,
            };
            var.type_0 = ShaderVarType_FromStr(varType.as_mut_ptr() as *const libc::c_char);
            if var.type_0 == 0_i32 {
                Fatal(
                    b"GLSL_Preprocess: Unknown shader variable type <%s> in directive:\n  %s\0"
                        as *const u8 as *const libc::c_char,
                    varType.as_mut_ptr(),
                    line,
                );
            }
            var.name = StrDup(varName.as_mut_ptr() as *const libc::c_char);
            var.index = -1_i32;
            if ((*this).vars_capacity == (*this).vars_size) as i32 as libc::c_long != 0 {
                (*this).vars_capacity = if (*this).vars_capacity != 0 {
                    (*this).vars_capacity * 2_i32
                } else {
                    1_i32
                };
                let mut elemSize: usize = std::mem::size_of::<ShaderVar>();
                let mut pData: *mut *mut libc::c_void =
                    &mut (*this).vars_data as *mut *mut ShaderVar as *mut *mut libc::c_void;
                *pData = MemRealloc(
                    (*this).vars_data as *mut libc::c_void,
                    ((*this).vars_capacity as usize).wrapping_mul(elemSize),
                );
            }
            let fresh13 = (*this).vars_size;
            (*this).vars_size += 1;
            *((*this).vars_data).offset(fresh13 as isize) = var;
        } else {
            Fatal(
                b"GLSL_Preprocess: Failed to parse directive:\n  %s\0" as *const u8
                    as *const libc::c_char,
                line,
            );
        }

        let mut prev_0: *const libc::c_char = code;
        code = StrSub(
            code,
            begin,
            end_0,
            b"\0" as *const u8 as *const libc::c_char,
        );
        StrFree(prev_0);
        StrFree(line);
    }
    code
}

unsafe extern "C" fn Shader_BindVariables(mut this: *mut Shader) {
    let mut i: i32 = 0_i32;
    while i < (*this).vars_size {
        let mut var: *mut ShaderVar = ((*this).vars_data).offset(i as isize);
        (*var).index = gl::GetUniformLocation((*this).program, (*var).name);
        if (*var).index < 0_i32 {
            Warn(
                b"Shader_BindVariables: Automatic shader variable <%s> does not exist in shader <%s>\0"
                    as *const u8 as *const libc::c_char,
                (*var).name,
                (*this).name,
            );
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn Shader_Create(
    mut vs: *const libc::c_char,
    mut fs: *const libc::c_char,
) -> *mut Shader {
    let mut this: *mut Shader = MemAlloc(std::mem::size_of::<Shader>()) as *mut Shader;
    (*this)._refCount = 1_u32;
    (*this).vars_capacity = 0_i32;
    (*this).vars_size = 0_i32;
    (*this).vars_data = std::ptr::null_mut();
    vs = GLSL_Preprocess(
        StrReplace(
            vs,
            b"\r\n\0" as *const u8 as *const libc::c_char,
            b"\n\0" as *const u8 as *const libc::c_char,
        ),
        this,
    );
    fs = GLSL_Preprocess(
        StrReplace(
            fs,
            b"\r\n\0" as *const u8 as *const libc::c_char,
            b"\n\0" as *const u8 as *const libc::c_char,
        ),
        this,
    );
    (*this).vs = CreateGLShader(vs, gl::VERTEX_SHADER);
    (*this).fs = CreateGLShader(fs, gl::FRAGMENT_SHADER);
    (*this).program = CreateGLProgram((*this).vs, (*this).fs);
    (*this).texIndex = 1_u32;
    (*this).name = StrFormat(
        b"[anonymous shader @ %p]\0" as *const u8 as *const libc::c_char,
        this,
    );
    StrFree(vs);
    StrFree(fs);
    Shader_BindVariables(this);
    this
}

#[no_mangle]
pub unsafe extern "C" fn Shader_Load(
    mut vName: *const libc::c_char,
    mut fName: *const libc::c_char,
) -> *mut Shader {
    let mut this: *mut Shader = MemAlloc(std::mem::size_of::<Shader>()) as *mut Shader;
    (*this)._refCount = 1_u32;
    (*this).vars_capacity = 0_i32;
    (*this).vars_size = 0_i32;
    (*this).vars_data = std::ptr::null_mut();
    let mut vs: *const libc::c_char = GLSL_Load(vName, this);
    let mut fs: *const libc::c_char = GLSL_Load(fName, this);
    (*this).vs = CreateGLShader(vs, gl::VERTEX_SHADER);
    (*this).fs = CreateGLShader(fs, gl::FRAGMENT_SHADER);
    (*this).program = CreateGLProgram((*this).vs, (*this).fs);
    (*this).texIndex = 1_u32;
    (*this).name = StrFormat(
        b"[vs: %s , fs: %s]\0" as *const u8 as *const libc::c_char,
        vName,
        fName,
    );
    Shader_BindVariables(this);
    this
}

#[no_mangle]
pub unsafe extern "C" fn Shader_Acquire(mut this: *mut Shader) {
    (*this)._refCount = ((*this)._refCount).wrapping_add(1);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_Free(mut this: *mut Shader) {
    if !this.is_null() && {
        (*this)._refCount = ((*this)._refCount).wrapping_sub(1);
        (*this)._refCount <= 0_u32
    } {
        gl::DeleteShader((*this).vs);
        gl::DeleteShader((*this).fs);
        gl::DeleteProgram((*this).program);
        MemFree((*this).vars_data as *const libc::c_void);
        StrFree((*this).name);
        MemFree(this as *const libc::c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Shader_ToShaderState(mut this: *mut Shader) -> *mut ShaderState {
    ShaderState_Create(this)
}

#[no_mangle]
pub unsafe extern "C" fn Shader_Start(mut this: *mut Shader) {
    Profiler_Begin(
        (*std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"Shader_Start\0")).as_ptr(),
    );
    gl::UseProgram((*this).program);
    current = this;
    (*this).texIndex = 1_u32;

    /* Fetch & bind automatic variables from the shader var stack. */
    let mut i: i32 = 0_i32;
    while i < (*this).vars_size {
        let mut var: *mut ShaderVar = ((*this).vars_data).offset(i as isize);
        if !((*var).index < 0_i32) {
            let mut pValue: *mut libc::c_void = ShaderVar_Get((*var).name, (*var).type_0);
            if pValue.is_null() {
                Fatal(
                    b"Shader_Start: Shader variable stack does not contain variable <%s>\0"
                        as *const u8 as *const libc::c_char,
                    (*var).name,
                );
            }

            match (*var).type_0 {
                1 => {
                    let mut value: f32 = *(pValue as *mut f32);
                    gl::Uniform1f((*var).index, value);
                }
                2 => {
                    let mut value_0 = *(pValue as *mut Vec2);
                    gl::Uniform2f((*var).index, value_0.x, value_0.y);
                }
                3 => {
                    let mut value_1: Vec3 = *(pValue as *mut Vec3);
                    gl::Uniform3f((*var).index, value_1.x, value_1.y, value_1.z);
                }
                4 => {
                    let mut value_2: Vec4 = *(pValue as *mut Vec4);
                    gl::Uniform4f((*var).index, value_2.x, value_2.y, value_2.z, value_2.w);
                }
                5 => {
                    let mut value_3: i32 = *(pValue as *mut i32);
                    gl::Uniform1i((*var).index, value_3);
                }
                6 => {
                    let mut value_4: IVec2 = *(pValue as *mut IVec2);
                    gl::Uniform2i((*var).index, value_4.x, value_4.y);
                }
                7 => {
                    let mut value_5: IVec3 = *(pValue as *mut IVec3);
                    gl::Uniform3i((*var).index, value_5.x, value_5.y, value_5.z);
                }
                8 => {
                    let mut value_6: IVec4 = *(pValue as *mut IVec4);
                    gl::Uniform4i((*var).index, value_6.x, value_6.y, value_6.z, value_6.w);
                }
                9 => {
                    Shader_ISetMatrix((*var).index, *(pValue as *mut *mut Matrix));
                }
                10 => {
                    Shader_ISetTex1D((*var).index, *(pValue as *mut *mut Tex1D));
                }
                11 => {
                    Shader_ISetTex2D((*var).index, *(pValue as *mut *mut Tex2D));
                }
                12 => {
                    Shader_ISetTex3D((*var).index, *(pValue as *mut *mut Tex3D));
                }
                13 => {
                    Shader_ISetTexCube((*var).index, *(pValue as *mut *mut TexCube));
                }
                _ => {}
            }
        }
        i += 1;
    }
    Profiler_End();
}

#[no_mangle]
pub unsafe extern "C" fn Shader_Stop(mut _s: *mut Shader) {
    gl::UseProgram(0);
    current = std::ptr::null_mut();
}

unsafe extern "C" fn ShaderCache_FreeElem(
    mut _s: *const libc::c_char,
    mut data: *mut libc::c_void,
) {
    MemFree(data);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_ClearCache() {
    if !cache.is_null() {
        StrMap_FreeEx(
            cache,
            Some(
                ShaderCache_FreeElem
                    as unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> (),
            ),
        );
        cache = std::ptr::null_mut();
    }
}

#[no_mangle]
pub unsafe extern "C" fn Shader_GetHandle(mut this: *mut Shader) -> u32 {
    (*this).program
}

#[no_mangle]
pub unsafe extern "C" fn Shader_GetVariable(
    mut this: *mut Shader,
    mut name: *const libc::c_char,
) -> i32 {
    let mut index: i32 = gl::GetUniformLocation((*this).program, name);
    if index == -1_i32 {
        Fatal(
            b"Shader_GetVariable: Shader <%s> has no variable <%s>\0" as *const u8
                as *const libc::c_char,
            (*this).name,
            name,
        );
    }
    index
}

#[no_mangle]
pub unsafe extern "C" fn Shader_HasVariable(
    mut this: *mut Shader,
    mut name: *const libc::c_char,
) -> bool {
    gl::GetUniformLocation((*this).program, name) > -1_i32
}

#[no_mangle]
pub unsafe extern "C" fn Shader_ResetTexIndex() {
    (*current).texIndex = 1_u32;
}

#[no_mangle]
pub unsafe extern "C" fn Shader_SetFloat(mut name: *const libc::c_char, mut value: f32) {
    gl::Uniform1f(GetUniformIndex(current, name), value);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_ISetFloat(mut index: i32, mut value: f32) {
    gl::Uniform1f(index, value);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_SetFloat2(mut name: *const libc::c_char, mut x: f32, mut y: f32) {
    gl::Uniform2f(GetUniformIndex(current, name), x, y);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_ISetFloat2(mut index: i32, mut x: f32, mut y: f32) {
    gl::Uniform2f(index, x, y);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_SetFloat3(
    mut name: *const libc::c_char,
    mut x: f32,
    mut y: f32,
    mut z: f32,
) {
    gl::Uniform3f(GetUniformIndex(current, name), x, y, z);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_ISetFloat3(mut index: i32, mut x: f32, mut y: f32, mut z: f32) {
    gl::Uniform3f(index, x, y, z);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_SetFloat4(
    mut name: *const libc::c_char,
    mut x: f32,
    mut y: f32,
    mut z: f32,
    mut w: f32,
) {
    gl::Uniform4f(GetUniformIndex(current, name), x, y, z, w);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_ISetFloat4(
    mut index: i32,
    mut x: f32,
    mut y: f32,
    mut z: f32,
    mut w: f32,
) {
    gl::Uniform4f(index, x, y, z, w);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_SetInt(mut name: *const libc::c_char, mut value: i32) {
    gl::Uniform1i(GetUniformIndex(current, name), value);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_ISetInt(mut index: i32, mut value: i32) {
    gl::Uniform1i(index, value);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_SetMatrix(mut name: *const libc::c_char, mut value: *mut Matrix) {
    gl::UniformMatrix4fv(
        GetUniformIndex(current, name),
        1_i32,
        gl::TRUE,
        value as *mut f32,
    );
}

#[no_mangle]
pub unsafe extern "C" fn Shader_SetMatrixT(mut name: *const libc::c_char, mut value: *mut Matrix) {
    gl::UniformMatrix4fv(
        GetUniformIndex(current, name),
        1_i32,
        gl::FALSE,
        value as *mut f32,
    );
}

#[no_mangle]
pub unsafe extern "C" fn Shader_ISetMatrix(mut index: i32, mut value: *mut Matrix) {
    gl::UniformMatrix4fv(index, 1_i32, gl::TRUE, value as *mut f32);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_ISetMatrixT(mut index: i32, mut value: *mut Matrix) {
    gl::UniformMatrix4fv(index, 1_i32, gl::FALSE, value as *mut f32);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_SetTex1D(mut name: *const libc::c_char, mut value: *mut Tex1D) {
    gl::Uniform1i(GetUniformIndex(current, name), (*current).texIndex as i32);
    let fresh14 = (*current).texIndex;
    (*current).texIndex = ((*current).texIndex).wrapping_add(1);
    gl::ActiveTexture((gl::TEXTURE0).wrapping_add(fresh14));
    gl::BindTexture(gl::TEXTURE_1D, Tex1D_GetHandle(value));
    gl::ActiveTexture(gl::TEXTURE0);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_ISetTex1D(mut index: i32, mut value: *mut Tex1D) {
    gl::Uniform1i(index, (*current).texIndex as i32);
    let fresh15 = (*current).texIndex;
    (*current).texIndex = ((*current).texIndex).wrapping_add(1);
    gl::ActiveTexture((gl::TEXTURE0).wrapping_add(fresh15));
    gl::BindTexture(gl::TEXTURE_1D, Tex1D_GetHandle(value));
    gl::ActiveTexture(gl::TEXTURE0);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_SetTex2D(mut name: *const libc::c_char, mut value: *mut Tex2D) {
    gl::Uniform1i(GetUniformIndex(current, name), (*current).texIndex as i32);
    let fresh16 = (*current).texIndex;
    (*current).texIndex = ((*current).texIndex).wrapping_add(1);
    gl::ActiveTexture((gl::TEXTURE0).wrapping_add(fresh16));
    gl::BindTexture(gl::TEXTURE_2D, Tex2D_GetHandle(value));
    gl::ActiveTexture(gl::TEXTURE0);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_ISetTex2D(mut index: i32, mut value: *mut Tex2D) {
    gl::Uniform1i(index, (*current).texIndex as i32);
    let fresh17 = (*current).texIndex;
    (*current).texIndex = ((*current).texIndex).wrapping_add(1);
    gl::ActiveTexture((gl::TEXTURE0).wrapping_add(fresh17));
    gl::BindTexture(gl::TEXTURE_2D, Tex2D_GetHandle(value));
    gl::ActiveTexture(gl::TEXTURE0);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_SetTex3D(mut name: *const libc::c_char, mut value: *mut Tex3D) {
    gl::Uniform1i(GetUniformIndex(current, name), (*current).texIndex as i32);
    let fresh18 = (*current).texIndex;
    (*current).texIndex = ((*current).texIndex).wrapping_add(1);
    gl::ActiveTexture((gl::TEXTURE0).wrapping_add(fresh18));
    gl::BindTexture(gl::TEXTURE_3D, Tex3D_GetHandle(value));
    gl::ActiveTexture(gl::TEXTURE0);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_ISetTex3D(mut index: i32, mut value: *mut Tex3D) {
    gl::Uniform1i(index, (*current).texIndex as i32);
    let fresh19 = (*current).texIndex;
    (*current).texIndex = ((*current).texIndex).wrapping_add(1);
    gl::ActiveTexture((gl::TEXTURE0).wrapping_add(fresh19));
    gl::BindTexture(gl::TEXTURE_3D, Tex3D_GetHandle(value));
    gl::ActiveTexture(gl::TEXTURE0);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_SetTexCube(mut name: *const libc::c_char, mut value: *mut TexCube) {
    gl::Uniform1i(GetUniformIndex(current, name), (*current).texIndex as i32);
    let fresh20 = (*current).texIndex;
    (*current).texIndex = ((*current).texIndex).wrapping_add(1);
    gl::ActiveTexture((gl::TEXTURE0).wrapping_add(fresh20));
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, TexCube_GetHandle(value));
    gl::ActiveTexture(gl::TEXTURE0);
}

#[no_mangle]
pub unsafe extern "C" fn Shader_ISetTexCube(mut index: i32, mut value: *mut TexCube) {
    gl::Uniform1i(index, (*current).texIndex as i32);
    let fresh21 = (*current).texIndex;
    (*current).texIndex = ((*current).texIndex).wrapping_add(1);
    gl::ActiveTexture((gl::TEXTURE0).wrapping_add(fresh21));
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, TexCube_GetHandle(value));
    gl::ActiveTexture(gl::TEXTURE0);
}
