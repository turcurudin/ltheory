use ::libc;
use glam::Vec3;
use std::ffi::VaListImpl;
use crate::internal::Memory::*;
extern "C" {
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::size_t,
        _: *const libc::c_char,
        _: __builtin_va_list,
    ) -> libc::c_int;
}
pub type __builtin_va_list = *mut libc::c_char;
pub type int32_t = libc::c_int;
pub type uint32_t = libc::c_uint;
pub type cstr = *const libc::c_char;
pub type int32 = int32_t;
pub type uint32 = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StrBuffer {
    pub data: *mut libc::c_char,
    pub size: uint32,
    pub capacity: uint32,
}
pub type va_list = __builtin_va_list;




#[inline]
unsafe extern "C" fn StrBuffer_GrowTo(mut this: *mut StrBuffer, mut newSize: uint32) {
    if (newSize > (*this).capacity) as libc::c_long != 0 {
        while (*this).capacity < newSize {
            (*this)
                .capacity = ((*this).capacity as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint) as uint32 as uint32;
        }
        (*this)
            .data = MemRealloc(
            (*this).data as *mut libc::c_void,
            ((*this).capacity).wrapping_add(1 as libc::c_int as libc::c_uint) as libc::size_t,
        ) as *mut libc::c_char;
        MemSet(
            ((*this).data).offset((*this).size as isize) as *mut libc::c_void,
            0 as libc::c_int,
            ((*this).capacity)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_sub((*this).size) as libc::size_t,
        );
    }
}
#[inline]
unsafe extern "C" fn StrBuffer_AppendData(
    mut this: *mut StrBuffer,
    mut data: *const libc::c_void,
    mut len: uint32,
) {
    StrBuffer_GrowTo(this, ((*this).size).wrapping_add(len));
    MemCpy(
        ((*this).data).offset((*this).size as isize) as *mut libc::c_void,
        data,
        len as usize,
    );
    (*this)
        .size = ((*this).size as libc::c_uint).wrapping_add(len) as uint32 as uint32;
}
#[no_mangle]
pub unsafe extern "C" fn StrBuffer_Create(mut capacity: uint32) -> *mut StrBuffer {
    let mut this: *mut StrBuffer = MemAlloc(
        ::core::mem::size_of::<StrBuffer>() as usize,
    ) as *mut StrBuffer;
    (*this)
        .data = MemAllocZero(
        capacity.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::size_t,
    ) as *mut libc::c_char;
    (*this).size = 0 as libc::c_int as uint32;
    (*this).capacity = capacity;
    return this;
}
#[no_mangle]
pub unsafe extern "C" fn StrBuffer_FromStr(
    mut s: *const libc::c_char,
) -> *mut StrBuffer {
    let mut len: uint32 = StrLen(s) as uint32;
    let mut this: *mut StrBuffer = StrBuffer_Create(len);
    (*this).size = len;
    MemCpy((*this).data as *mut libc::c_void, s as *const libc::c_void, len as usize);
    return this;
}
#[no_mangle]
pub unsafe extern "C" fn StrBuffer_Free(mut this: *mut StrBuffer) {
    MemFree((*this).data as *const libc::c_void);
    MemFree(this as *const libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn StrBuffer_Append(
    mut this: *mut StrBuffer,
    mut other: *mut StrBuffer,
) {
    StrBuffer_AppendData(this, (*other).data as *const libc::c_void, (*other).size);
}
#[no_mangle]
pub unsafe extern "C" fn StrBuffer_AppendStr(
    mut this: *mut StrBuffer,
    mut other: *const libc::c_char,
) {
    StrBuffer_AppendData(this, other as *const libc::c_void, StrLen(other) as uint32);
}
#[inline]
unsafe extern "C" fn StrBuffer_SetImpl(
    mut this: *mut StrBuffer,
    mut format: cstr,
    mut args: va_list,
) -> int32 {
    let mut newSize: int32 = vsnprintf(
        (*this).data,
        ((*this).capacity).wrapping_add(1) as usize,
        format,
        args,
    );
    if (newSize as uint32 <= (*this).capacity) as libc::c_long != 0 {
        (*this).size = newSize as uint32;
        return 0 as libc::c_int;
    } else {
        return (newSize as libc::c_uint).wrapping_sub((*this).capacity) as int32
    };
}
#[no_mangle]
pub unsafe extern "C" fn StrBuffer_Set(
    mut this: *mut StrBuffer,
    mut format: cstr,
    mut args: ...
) {
    let mut args_0: va_list = 0 as *mut libc::c_char;
    args_0 = &args as *const VaListImpl as va_list;
    let mut neededSpace: int32 = StrBuffer_SetImpl(this, format, args_0);
    if (neededSpace > 0 as libc::c_int) as libc::c_long != 0 {
        StrBuffer_GrowTo(
            this,
            ((*this).capacity).wrapping_add(neededSpace as libc::c_uint),
        );
        let mut args2: va_list = 0 as *mut libc::c_char;
        args2 = &args as *const VaListImpl as va_list;
        neededSpace = StrBuffer_SetImpl(this, format, args_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn StrBuffer_Clone(mut other: *mut StrBuffer) -> *mut StrBuffer {
    let mut this: *mut StrBuffer = StrBuffer_Create((*other).size);
    MemCpy(
        (*this).data as *mut libc::c_void,
        (*other).data as *const libc::c_void,
        (*other).size as libc::size_t,
    );
    (*this).size = (*other).size;
    return this;
}
#[no_mangle]
pub unsafe extern "C" fn StrBuffer_GetData(mut this: *mut StrBuffer) -> cstr {
    return (*this).data as cstr;
}
