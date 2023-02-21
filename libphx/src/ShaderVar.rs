use ::libc;
use glam::Vec3;
use crate::internal::Memory::*;
use glam::Vec2;
extern "C" {
    pub type StrMap;
    pub type Tex1D;
    pub type Tex2D;
    pub type Tex3D;
    pub type TexCube;
    pub type Matrix;
    fn Fatal(_: cstr, _: ...);
    fn ShaderVarType_GetName(_: ShaderVarType) -> cstr;
    fn ShaderVarType_GetSize(_: ShaderVarType) -> i32;
    fn StrMap_Create(initCapacity: u32) -> *mut StrMap;
    fn StrMap_Free(_: *mut StrMap);
    fn StrMap_Get(_: *mut StrMap, key: cstr) -> *mut libc::c_void;
    fn StrMap_Set(_: *mut StrMap, key: cstr, val: *mut libc::c_void);
}
pub type cstr = *const libc::c_char;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
pub type ShaderVarType = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VarStack {
    pub type_0: ShaderVarType,
    pub size: i32,
    pub capacity: i32,
    pub elemSize: i32,
    pub data: *mut libc::c_void,
}


static mut varMap: *mut StrMap = 0 as *const StrMap as *mut StrMap;
#[inline]
unsafe extern "C" fn ShaderVar_GetStack(
    mut var: cstr,
    mut type_0: ShaderVarType,
) -> *mut VarStack {
    let mut this: *mut VarStack = StrMap_Get(varMap, var) as *mut VarStack;
    if this.is_null() {
        if type_0 == 0 {
            return 0 as *mut VarStack;
        }
        this = MemAlloc(::core::mem::size_of::<VarStack>())
            as *mut VarStack;
        (*this).type_0 = type_0;
        (*this).size = 0 as i32;
        (*this).capacity = 4 as i32;
        (*this).elemSize = ShaderVarType_GetSize(type_0);
        (*this).data = MemAlloc(((*this).capacity * (*this).elemSize) as usize);
        StrMap_Set(varMap, var, this as *mut libc::c_void);
    }
    if type_0 != 0 && (*this).type_0 != type_0 {
        Fatal(
            b"ShaderVar_GetStack: Attempting to get stack of type <%s> for shader variable <%s> when existing stack has type <%s>\0"
                as *const u8 as *const libc::c_char,
            ShaderVarType_GetName(type_0),
            var,
            ShaderVarType_GetName((*this).type_0),
        );
    }
    return this;
}
#[inline]
unsafe extern "C" fn ShaderVar_Push(
    mut var: cstr,
    mut type_0: ShaderVarType,
    mut value: *const libc::c_void,
) {
    let mut this: *mut VarStack = ShaderVar_GetStack(var, type_0);
    if (*this).size == (*this).capacity {
        (*this).capacity *= 2 as i32;
        (*this)
            .data = MemRealloc(
            (*this).data,
            ((*this).capacity * (*this).elemSize) as usize,
        );
    }
    MemCpy(
        ((*this).data as *mut libc::c_char)
            .offset(((*this).size * (*this).elemSize) as isize) as *mut libc::c_void,
        value,
        (*this).elemSize as usize,
    );
    (*this).size += 1;
}
#[no_mangle]
pub unsafe extern "C" fn ShaderVar_Init() {
    varMap = StrMap_Create(16 as i32 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn ShaderVar_Free() {
    StrMap_Free(varMap);
    varMap = 0 as *mut StrMap;
}
#[no_mangle]
pub unsafe extern "C" fn ShaderVar_Get(
    mut name: cstr,
    mut type_0: ShaderVarType,
) -> *mut libc::c_void {
    let mut this: *mut VarStack = ShaderVar_GetStack(name, 0 as i32);
    if this.is_null() || (*this).size == 0 as i32 {
        return 0 as *mut libc::c_void;
    }
    if type_0 != 0 && (*this).type_0 != type_0 {
        Fatal(
            b"ShaderVar_Get: Attempting to get variable <%s> with type <%s> when existing stack has type <%s>\0"
                as *const u8 as *const libc::c_char,
            name,
            ShaderVarType_GetName(type_0),
            ShaderVarType_GetName((*this).type_0),
        );
    }
    return ((*this).data as *mut libc::c_char)
        .offset(((*this).elemSize * ((*this).size - 1 as i32)) as isize)
        as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn ShaderVar_PushFloat(mut name: cstr, mut x: f32) {
    ShaderVar_Push(
        name,
        0x1 as i32,
        &mut x as *mut f32 as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ShaderVar_PushFloat2(
    mut name: cstr,
    mut x: f32,
    mut y: f32,
) {
    let mut value = Vec2::new(x, y);
    ShaderVar_Push(
        name,
        0x2 as i32,
        &mut value as *mut Vec2 as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ShaderVar_PushFloat3(
    mut name: cstr,
    mut x: f32,
    mut y: f32,
    mut z: f32,
) {
    let mut value: Vec3 =  Vec3 { x: x, y: y, z: z };
    ShaderVar_Push(
        name,
        0x3 as i32,
        &mut value as *mut Vec3 as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ShaderVar_PushFloat4(
    mut name: cstr,
    mut x: f32,
    mut y: f32,
    mut z: f32,
    mut w: f32,
) {
    let mut value: Vec4f =  Vec4f { x: x, y: y, z: z, w: w };
    ShaderVar_Push(
        name,
        0x4 as i32,
        &mut value as *mut Vec4f as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ShaderVar_PushInt(mut name: cstr, mut x: i32) {
    let mut value: i32 = x;
    ShaderVar_Push(
        name,
        0x5 as i32,
        &mut value as *mut i32 as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ShaderVar_PushMatrix(mut name: cstr, mut x: *mut Matrix) {
    ShaderVar_Push(
        name,
        0x9 as i32,
        &mut x as *mut *mut Matrix as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ShaderVar_PushTex1D(mut name: cstr, mut x: *mut Tex1D) {
    ShaderVar_Push(
        name,
        0xa as i32,
        &mut x as *mut *mut Tex1D as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ShaderVar_PushTex2D(mut name: cstr, mut x: *mut Tex2D) {
    ShaderVar_Push(
        name,
        0xb as i32,
        &mut x as *mut *mut Tex2D as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ShaderVar_PushTex3D(mut name: cstr, mut x: *mut Tex3D) {
    ShaderVar_Push(
        name,
        0xc as i32,
        &mut x as *mut *mut Tex3D as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ShaderVar_PushTexCube(mut name: cstr, mut x: *mut TexCube) {
    ShaderVar_Push(
        name,
        0xd as i32,
        &mut x as *mut *mut TexCube as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ShaderVar_Pop(mut name: cstr) {
    let mut this: *mut VarStack = ShaderVar_GetStack(name, 0 as i32);
    if this.is_null() {
        Fatal(
            b"ShaderVar_Pop: Attempting to pop nonexistent stack <%s>\0" as *const u8
                as *const libc::c_char,
            name,
        );
    }
    if (*this).size == 0 as i32 {
        Fatal(
            b"ShaderVar_Pop: Attempting to pop empty stack <%s>\0" as *const u8
                as *const libc::c_char,
            name,
        );
    }
    (*this).size -= 1;
}
