use ::libc;
use crate::internal::Memory::*;
extern "C" {
    pub type Shader;
    pub type TexCube;
    pub type Tex3D;
    pub type Tex2D;
    pub type Tex1D;
    pub type Matrix;
    fn Fatal(_: cstr, _: ...);
    static mut __glewUniform1f: PFNGLUNIFORM1FPROC;
    static mut __glewUniform1i: PFNGLUNIFORM1IPROC;
    static mut __glewUniform2f: PFNGLUNIFORM2FPROC;
    static mut __glewUniform3f: PFNGLUNIFORM3FPROC;
    static mut __glewUniform4f: PFNGLUNIFORM4FPROC;
    fn Shader_Load(vertName: cstr, fragName: cstr) -> *mut Shader;
    fn Shader_Acquire(_: *mut Shader);
    fn Shader_Free(_: *mut Shader);
    fn Shader_Start(_: *mut Shader);
    fn Shader_Stop(_: *mut Shader);
    fn Shader_GetVariable(_: *mut Shader, _: cstr) -> libc::c_int;
    fn Shader_ISetMatrix(_: libc::c_int, _: *mut Matrix);
    fn Shader_ISetTex1D(_: libc::c_int, _: *mut Tex1D);
    fn Shader_ISetTex2D(_: libc::c_int, _: *mut Tex2D);
    fn Shader_ISetTex3D(_: libc::c_int, _: *mut Tex3D);
    fn Shader_ISetTexCube(_: libc::c_int, _: *mut TexCube);
    fn Tex1D_Acquire(_: *mut Tex1D);
    fn Tex1D_Free(_: *mut Tex1D);
    fn Tex2D_Acquire(_: *mut Tex2D);
    fn Tex2D_Free(_: *mut Tex2D);
    fn Tex3D_Acquire(_: *mut Tex3D);
    fn Tex3D_Free(_: *mut Tex3D);
    fn TexCube_Acquire(_: *mut TexCube);
    fn TexCube_Free(_: *mut TexCube);
}
pub type int32_t = libc::c_int;
pub type uint32_t = libc::c_uint;
pub type cstr = *const libc::c_char;
pub type int32 = int32_t;
pub type uint32 = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ShaderState {
    pub _refCount: uint32,
    pub shader: *mut Shader,
    pub elems_size: int32,
    pub elems_capacity: int32,
    pub elems_data: *mut Elem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elem {
    pub type_0: uint32,
    pub index: int32,
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub asFloat: libc::c_float,
    pub asFloat2: Vec2f,
    pub asFloat3: Vec3f,
    pub asFloat4: Vec4f,
    pub asInt: libc::c_int,
    pub asMatrix: *mut Matrix,
    pub asTex1D: *mut Tex1D,
    pub asTex2D: *mut Tex2D,
    pub asTex3D: *mut Tex3D,
    pub asTexCube: *mut TexCube,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec4f {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
    pub w: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec3f {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec2f {
    pub x: libc::c_float,
    pub y: libc::c_float,
}
pub type GLint = libc::c_int;
pub type GLfloat = libc::c_float;
pub type PFNGLUNIFORM1FPROC = Option::<unsafe extern "C" fn(GLint, GLfloat) -> ()>;
pub type PFNGLUNIFORM1IPROC = Option::<unsafe extern "C" fn(GLint, GLint) -> ()>;
pub type PFNGLUNIFORM2FPROC = Option::<
    unsafe extern "C" fn(GLint, GLfloat, GLfloat) -> (),
>;
pub type PFNGLUNIFORM3FPROC = Option::<
    unsafe extern "C" fn(GLint, GLfloat, GLfloat, GLfloat) -> (),
>;
pub type PFNGLUNIFORM4FPROC = Option::<
    unsafe extern "C" fn(GLint, GLfloat, GLfloat, GLfloat, GLfloat) -> (),
>;


#[inline]
unsafe extern "C" fn Vec2f_Create(mut x: libc::c_float, mut y: libc::c_float) -> Vec2f {
    let mut self_0: Vec2f = {
        let mut init = Vec2f { x: x, y: y };
        init
    };
    return self_0;
}
#[inline]
unsafe extern "C" fn Vec3f_Create(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut z: libc::c_float,
) -> Vec3f {
    let mut self_0: Vec3f = {
        let mut init = Vec3f { x: x, y: y, z: z };
        init
    };
    return self_0;
}
#[inline]
unsafe extern "C" fn Vec4f_Create(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut z: libc::c_float,
    mut w: libc::c_float,
) -> Vec4f {
    let mut self_0: Vec4f = {
        let mut init = Vec4f { x: x, y: y, z: z, w: w };
        init
    };
    return self_0;
}
#[no_mangle]
pub static mut ElemType_Float: uint32 = 1 as libc::c_int as uint32;
#[no_mangle]
pub static mut ElemType_Float2: uint32 = 2 as libc::c_int as uint32;
#[no_mangle]
pub static mut ElemType_Float3: uint32 = 3 as libc::c_int as uint32;
#[no_mangle]
pub static mut ElemType_Float4: uint32 = 4 as libc::c_int as uint32;
#[no_mangle]
pub static mut ElemType_Int: uint32 = 5 as libc::c_int as uint32;
#[no_mangle]
pub static mut ElemType_Matrix: uint32 = 6 as libc::c_int as uint32;
#[no_mangle]
pub static mut ElemType_Tex1D: uint32 = 7 as libc::c_int as uint32;
#[no_mangle]
pub static mut ElemType_Tex2D: uint32 = 8 as libc::c_int as uint32;
#[no_mangle]
pub static mut ElemType_Tex3D: uint32 = 9 as libc::c_int as uint32;
#[no_mangle]
pub static mut ElemType_TexCube: uint32 = 10 as libc::c_int as uint32;
#[no_mangle]
pub unsafe extern "C" fn ShaderState_Create(
    mut shader: *mut Shader,
) -> *mut ShaderState {
    let mut self_0: *mut ShaderState = MemAlloc(
        ::core::mem::size_of::<ShaderState>() as usize,
    ) as *mut ShaderState;
    (*self_0)._refCount = 1 as libc::c_int as uint32;
    (*self_0).elems_capacity = 0 as libc::c_int;
    (*self_0).elems_size = 0 as libc::c_int;
    (*self_0).elems_data = 0 as *mut Elem;
    Shader_Acquire(shader);
    (*self_0).shader = shader;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn ShaderState_Acquire(mut self_0: *mut ShaderState) {
    (*self_0)._refCount = ((*self_0)._refCount).wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn ShaderState_Free(mut self_0: *mut ShaderState) {
    if !self_0.is_null()
        && {
            (*self_0)._refCount = ((*self_0)._refCount).wrapping_sub(1);
            (*self_0)._refCount <= 0 as libc::c_int as libc::c_uint
        }
    {
        let mut e: *mut Elem = (*self_0).elems_data;
        let mut __iterend: *mut Elem = ((*self_0).elems_data)
            .offset((*self_0).elems_size as isize);
        while e < __iterend {
            match (*e).type_0 {
                7 => {
                    Tex1D_Free((*e).data.asTex1D);
                }
                8 => {
                    Tex2D_Free((*e).data.asTex2D);
                }
                9 => {
                    Tex3D_Free((*e).data.asTex3D);
                }
                10 => {
                    TexCube_Free((*e).data.asTexCube);
                }
                _ => {}
            }
            e = e.offset(1);
        }
        Shader_Free((*self_0).shader);
        MemFree((*self_0).elems_data as *const libc::c_void);
        MemFree(self_0 as *const libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ShaderState_FromShaderLoad(
    mut vertName: cstr,
    mut fragName: cstr,
) -> *mut ShaderState {
    let mut shader: *mut Shader = Shader_Load(vertName, fragName);
    let mut self_0: *mut ShaderState = ShaderState_Create(shader);
    Shader_Free(shader);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetFloat(
    mut self_0: *mut ShaderState,
    mut name: cstr,
    mut x: libc::c_float,
) {
    let mut elem: Elem = {
        let mut init = Elem {
            type_0: ElemType_Float,
            index: Shader_GetVariable((*self_0).shader, name),
            data: C2RustUnnamed { asFloat: 0. },
        };
        init
    };
    elem.data.asFloat = x;
    if ((*self_0).elems_capacity == (*self_0).elems_size) as libc::c_int as libc::c_long
        != 0
    {
        (*self_0)
            .elems_capacity = if (*self_0).elems_capacity != 0 {
            (*self_0).elems_capacity * 2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let mut elemSize: usize = ::core::mem::size_of::<Elem>() as usize;
        let mut pData: *mut *mut libc::c_void = &mut (*self_0).elems_data
            as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*self_0).elems_data as *mut libc::c_void,
            ((*self_0).elems_capacity as usize).wrapping_mul(elemSize as usize),
        );
    }
    let fresh0 = (*self_0).elems_size;
    (*self_0).elems_size = (*self_0).elems_size + 1;
    *((*self_0).elems_data).offset(fresh0 as isize) = elem;
}
#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetFloat2(
    mut self_0: *mut ShaderState,
    mut name: cstr,
    mut x: libc::c_float,
    mut y: libc::c_float,
) {
    let mut elem: Elem = {
        let mut init = Elem {
            type_0: ElemType_Float2,
            index: Shader_GetVariable((*self_0).shader, name),
            data: C2RustUnnamed { asFloat: 0. },
        };
        init
    };
    elem.data.asFloat2 = Vec2f_Create(x, y);
    if ((*self_0).elems_capacity == (*self_0).elems_size) as libc::c_int as libc::c_long
        != 0
    {
        (*self_0)
            .elems_capacity = if (*self_0).elems_capacity != 0 {
            (*self_0).elems_capacity * 2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let mut elemSize: usize = ::core::mem::size_of::<Elem>() as usize;
        let mut pData: *mut *mut libc::c_void = &mut (*self_0).elems_data
            as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*self_0).elems_data as *mut libc::c_void,
            ((*self_0).elems_capacity as usize).wrapping_mul(elemSize as usize),
        );
    }
    let fresh1 = (*self_0).elems_size;
    (*self_0).elems_size = (*self_0).elems_size + 1;
    *((*self_0).elems_data).offset(fresh1 as isize) = elem;
}
#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetFloat3(
    mut self_0: *mut ShaderState,
    mut name: cstr,
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut z: libc::c_float,
) {
    let mut elem: Elem = {
        let mut init = Elem {
            type_0: ElemType_Float3,
            index: Shader_GetVariable((*self_0).shader, name),
            data: C2RustUnnamed { asFloat: 0. },
        };
        init
    };
    elem.data.asFloat3 = Vec3f_Create(x, y, z);
    if ((*self_0).elems_capacity == (*self_0).elems_size) as libc::c_int as libc::c_long
        != 0
    {
        (*self_0)
            .elems_capacity = if (*self_0).elems_capacity != 0 {
            (*self_0).elems_capacity * 2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let mut elemSize: usize = ::core::mem::size_of::<Elem>() as usize;
        let mut pData: *mut *mut libc::c_void = &mut (*self_0).elems_data
            as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*self_0).elems_data as *mut libc::c_void,
            ((*self_0).elems_capacity as usize).wrapping_mul(elemSize as usize),
        );
    }
    let fresh2 = (*self_0).elems_size;
    (*self_0).elems_size = (*self_0).elems_size + 1;
    *((*self_0).elems_data).offset(fresh2 as isize) = elem;
}
#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetFloat4(
    mut self_0: *mut ShaderState,
    mut name: cstr,
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut z: libc::c_float,
    mut w: libc::c_float,
) {
    let mut elem: Elem = {
        let mut init = Elem {
            type_0: ElemType_Float4,
            index: Shader_GetVariable((*self_0).shader, name),
            data: C2RustUnnamed { asFloat: 0. },
        };
        init
    };
    elem.data.asFloat4 = Vec4f_Create(x, y, z, w);
    if ((*self_0).elems_capacity == (*self_0).elems_size) as libc::c_int as libc::c_long
        != 0
    {
        (*self_0)
            .elems_capacity = if (*self_0).elems_capacity != 0 {
            (*self_0).elems_capacity * 2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let mut elemSize: usize = ::core::mem::size_of::<Elem>() as usize;
        let mut pData: *mut *mut libc::c_void = &mut (*self_0).elems_data
            as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*self_0).elems_data as *mut libc::c_void,
            ((*self_0).elems_capacity as usize).wrapping_mul(elemSize as usize),
        );
    }
    let fresh3 = (*self_0).elems_size;
    (*self_0).elems_size = (*self_0).elems_size + 1;
    *((*self_0).elems_data).offset(fresh3 as isize) = elem;
}
#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetInt(
    mut self_0: *mut ShaderState,
    mut name: cstr,
    mut x: libc::c_int,
) {
    let mut elem: Elem = {
        let mut init = Elem {
            type_0: ElemType_Int,
            index: Shader_GetVariable((*self_0).shader, name),
            data: C2RustUnnamed { asFloat: 0. },
        };
        init
    };
    elem.data.asInt = x;
    if ((*self_0).elems_capacity == (*self_0).elems_size) as libc::c_int as libc::c_long
        != 0
    {
        (*self_0)
            .elems_capacity = if (*self_0).elems_capacity != 0 {
            (*self_0).elems_capacity * 2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let mut elemSize: usize = ::core::mem::size_of::<Elem>() as usize;
        let mut pData: *mut *mut libc::c_void = &mut (*self_0).elems_data
            as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*self_0).elems_data as *mut libc::c_void,
            ((*self_0).elems_capacity as usize).wrapping_mul(elemSize as usize),
        );
    }
    let fresh4 = (*self_0).elems_size;
    (*self_0).elems_size = (*self_0).elems_size + 1;
    *((*self_0).elems_data).offset(fresh4 as isize) = elem;
}
#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetMatrix(
    mut self_0: *mut ShaderState,
    mut name: cstr,
    mut x: *mut Matrix,
) {
    let mut elem: Elem = {
        let mut init = Elem {
            type_0: ElemType_Matrix,
            index: Shader_GetVariable((*self_0).shader, name),
            data: C2RustUnnamed { asFloat: 0. },
        };
        init
    };
    elem.data.asMatrix = x;
    if ((*self_0).elems_capacity == (*self_0).elems_size) as libc::c_int as libc::c_long
        != 0
    {
        (*self_0)
            .elems_capacity = if (*self_0).elems_capacity != 0 {
            (*self_0).elems_capacity * 2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let mut elemSize: usize = ::core::mem::size_of::<Elem>() as usize;
        let mut pData: *mut *mut libc::c_void = &mut (*self_0).elems_data
            as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*self_0).elems_data as *mut libc::c_void,
            ((*self_0).elems_capacity as usize).wrapping_mul(elemSize as usize),
        );
    }
    let fresh5 = (*self_0).elems_size;
    (*self_0).elems_size = (*self_0).elems_size + 1;
    *((*self_0).elems_data).offset(fresh5 as isize) = elem;
}
#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetTex1D(
    mut self_0: *mut ShaderState,
    mut name: cstr,
    mut x: *mut Tex1D,
) {
    Tex1D_Acquire(x);
    let mut elem: Elem = {
        let mut init = Elem {
            type_0: ElemType_Tex1D,
            index: Shader_GetVariable((*self_0).shader, name),
            data: C2RustUnnamed { asFloat: 0. },
        };
        init
    };
    elem.data.asTex1D = x;
    if ((*self_0).elems_capacity == (*self_0).elems_size) as libc::c_int as libc::c_long
        != 0
    {
        (*self_0)
            .elems_capacity = if (*self_0).elems_capacity != 0 {
            (*self_0).elems_capacity * 2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let mut elemSize: usize = ::core::mem::size_of::<Elem>() as usize;
        let mut pData: *mut *mut libc::c_void = &mut (*self_0).elems_data
            as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*self_0).elems_data as *mut libc::c_void,
            ((*self_0).elems_capacity as usize).wrapping_mul(elemSize as usize),
        );
    }
    let fresh6 = (*self_0).elems_size;
    (*self_0).elems_size = (*self_0).elems_size + 1;
    *((*self_0).elems_data).offset(fresh6 as isize) = elem;
}
#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetTex2D(
    mut self_0: *mut ShaderState,
    mut name: cstr,
    mut x: *mut Tex2D,
) {
    Tex2D_Acquire(x);
    let mut elem: Elem = {
        let mut init = Elem {
            type_0: ElemType_Tex2D,
            index: Shader_GetVariable((*self_0).shader, name),
            data: C2RustUnnamed { asFloat: 0. },
        };
        init
    };
    elem.data.asTex2D = x;
    if ((*self_0).elems_capacity == (*self_0).elems_size) as libc::c_int as libc::c_long
        != 0
    {
        (*self_0)
            .elems_capacity = if (*self_0).elems_capacity != 0 {
            (*self_0).elems_capacity * 2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let mut elemSize: usize = ::core::mem::size_of::<Elem>() as usize;
        let mut pData: *mut *mut libc::c_void = &mut (*self_0).elems_data
            as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*self_0).elems_data as *mut libc::c_void,
            ((*self_0).elems_capacity as usize).wrapping_mul(elemSize as usize),
        );
    }
    let fresh7 = (*self_0).elems_size;
    (*self_0).elems_size = (*self_0).elems_size + 1;
    *((*self_0).elems_data).offset(fresh7 as isize) = elem;
}
#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetTex3D(
    mut self_0: *mut ShaderState,
    mut name: cstr,
    mut x: *mut Tex3D,
) {
    Tex3D_Acquire(x);
    let mut elem: Elem = {
        let mut init = Elem {
            type_0: ElemType_Tex3D,
            index: Shader_GetVariable((*self_0).shader, name),
            data: C2RustUnnamed { asFloat: 0. },
        };
        init
    };
    elem.data.asTex3D = x;
    if ((*self_0).elems_capacity == (*self_0).elems_size) as libc::c_int as libc::c_long
        != 0
    {
        (*self_0)
            .elems_capacity = if (*self_0).elems_capacity != 0 {
            (*self_0).elems_capacity * 2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let mut elemSize: usize = ::core::mem::size_of::<Elem>() as usize;
        let mut pData: *mut *mut libc::c_void = &mut (*self_0).elems_data
            as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*self_0).elems_data as *mut libc::c_void,
            ((*self_0).elems_capacity as usize).wrapping_mul(elemSize as usize),
        );
    }
    let fresh8 = (*self_0).elems_size;
    (*self_0).elems_size = (*self_0).elems_size + 1;
    *((*self_0).elems_data).offset(fresh8 as isize) = elem;
}
#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetTexCube(
    mut self_0: *mut ShaderState,
    mut name: cstr,
    mut x: *mut TexCube,
) {
    TexCube_Acquire(x);
    let mut elem: Elem = {
        let mut init = Elem {
            type_0: ElemType_TexCube,
            index: Shader_GetVariable((*self_0).shader, name),
            data: C2RustUnnamed { asFloat: 0. },
        };
        init
    };
    elem.data.asTexCube = x;
    if ((*self_0).elems_capacity == (*self_0).elems_size) as libc::c_int as libc::c_long
        != 0
    {
        (*self_0)
            .elems_capacity = if (*self_0).elems_capacity != 0 {
            (*self_0).elems_capacity * 2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let mut elemSize: usize = ::core::mem::size_of::<Elem>() as usize;
        let mut pData: *mut *mut libc::c_void = &mut (*self_0).elems_data
            as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*self_0).elems_data as *mut libc::c_void,
            ((*self_0).elems_capacity as usize).wrapping_mul(elemSize as usize),
        );
    }
    let fresh9 = (*self_0).elems_size;
    (*self_0).elems_size = (*self_0).elems_size + 1;
    *((*self_0).elems_data).offset(fresh9 as isize) = elem;
}
#[no_mangle]
pub unsafe extern "C" fn ShaderState_Start(mut self_0: *mut ShaderState) {
    Shader_Start((*self_0).shader);
    let mut e: *mut Elem = (*self_0).elems_data;
    let mut __iterend: *mut Elem = ((*self_0).elems_data)
        .offset((*self_0).elems_size as isize);
    while e < __iterend {
        match (*e).type_0 {
            1 => {
                __glewUniform1f
                    .expect("non-null function pointer")((*e).index, (*e).data.asFloat);
            }
            2 => {
                __glewUniform2f
                    .expect(
                        "non-null function pointer",
                    )((*e).index, (*e).data.asFloat2.x, (*e).data.asFloat2.y);
            }
            3 => {
                __glewUniform3f
                    .expect(
                        "non-null function pointer",
                    )(
                    (*e).index,
                    (*e).data.asFloat3.x,
                    (*e).data.asFloat3.y,
                    (*e).data.asFloat3.z,
                );
            }
            4 => {
                __glewUniform4f
                    .expect(
                        "non-null function pointer",
                    )(
                    (*e).index,
                    (*e).data.asFloat4.x,
                    (*e).data.asFloat4.y,
                    (*e).data.asFloat4.z,
                    (*e).data.asFloat4.w,
                );
            }
            5 => {
                __glewUniform1i
                    .expect("non-null function pointer")((*e).index, (*e).data.asInt);
            }
            6 => {
                Shader_ISetMatrix((*e).index, (*e).data.asMatrix);
            }
            7 => {
                Shader_ISetTex1D((*e).index, (*e).data.asTex1D);
            }
            8 => {
                Shader_ISetTex2D((*e).index, (*e).data.asTex2D);
            }
            9 => {
                Shader_ISetTex3D((*e).index, (*e).data.asTex3D);
            }
            10 => {
                Shader_ISetTexCube((*e).index, (*e).data.asTexCube);
            }
            _ => {
                Fatal(
                    b"ShaderState_Start: Encountered invalid opcode\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        e = e.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ShaderState_Stop(mut self_0: *mut ShaderState) {
    Shader_Stop((*self_0).shader);
}
