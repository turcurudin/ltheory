use crate::internal::Memory::*;
use glam::Vec3;
use libc;
use std::fs;
extern "C" {
    pub type Bytes;
    fn Fatal(_: *const libc::c_char, _: ...);
    fn Bytes_Create(len: u32) -> *mut Bytes;
    fn Bytes_GetData(_: *mut Bytes) -> *mut libc::c_void;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct File {
    pub handle: *mut libc::FILE,
}

#[no_mangle]
pub unsafe extern "C" fn File_Exists(mut path: *const libc::c_char) -> bool {
    let mut f: *mut libc::FILE = libc::fopen(path, b"rb\0" as *const u8 as *const libc::c_char);
    if !f.is_null() {
        libc::fclose(f);
        return 1 as i32 != 0;
    }
    return 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn File_IsDir(path: *const libc::c_char) -> bool {
    let meta = fs::metadata(std::ffi::CStr::from_ptr(path).to_str().unwrap()).unwrap();
    return meta.is_dir();
}
unsafe extern "C" fn File_OpenMode(mut path: *const libc::c_char, mut mode: *const libc::c_char) -> *mut File {
    let mut handle: *mut libc::FILE = libc::fopen(path, mode);
    if handle.is_null() {
        return 0 as *mut File;
    }
    let mut this: *mut File = MemAlloc(::core::mem::size_of::<File>()) as *mut File;
    (*this).handle = handle;
    return this;
}
#[no_mangle]
pub unsafe extern "C" fn File_Create(mut path: *const libc::c_char) -> *mut File {
    return File_OpenMode(path, b"wb\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn File_Open(mut path: *const libc::c_char) -> *mut File {
    return File_OpenMode(path, b"ab\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn File_Close(mut this: *mut File) {
    libc::fclose((*this).handle);
    MemFree(this as *const libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn File_ReadBytes(mut path: *const libc::c_char) -> *mut Bytes {
    let mut file: *mut libc::FILE = libc::fopen(path, b"rb\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        return 0 as *mut Bytes;
    }
    libc::fseek(file, 0 as libc::c_long, libc::SEEK_END);
    let mut size: i64 = libc::ftell(file) as i64;
    if size == 0 as i64 {
        return 0 as *mut Bytes;
    }
    if size < 0 as i64 {
        Fatal(
            b"File_Read: failed to get size of file '%s'\0" as *const u8 as *const libc::c_char,
            path,
        );
    }
    libc::rewind(file);
    if size > sdl2_sys::UINT32_MAX as u32 as i64 {
        Fatal(
            b"File_Read: filesize of '%s' exceeds 32-bit capacity limit\0" as *const u8
                as *const libc::c_char,
            path,
        );
    }
    let mut buffer: *mut Bytes = Bytes_Create(size as u32);
    let mut result: usize = libc::fread(Bytes_GetData(buffer), size as usize, 1 as usize, file);
    if result != 1 as usize {
        Fatal(
            b"File_Read: failed to read correct number of bytes from '%s'\0" as *const u8
                as *const libc::c_char,
            path,
        );
    }
    libc::fclose(file);
    return buffer;
}

#[no_mangle]
pub unsafe extern "C" fn File_ReadCstr(mut path: *const libc::c_char) -> *const libc::c_char {
    let mut file: *mut libc::FILE = libc::fopen(path, b"rb\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        return 0 as *const libc::c_char;
    }
    libc::fseek(file, 0 as libc::c_long, 2 as i32);
    let mut size: i64 = libc::ftell(file) as i64;
    if size == 0 as i64 {
        return 0 as *const libc::c_char;
    }
    if size < 0 as i64 {
        Fatal(
            b"File_ReadAscii: failed to get size of file '%s'\0" as *const u8
                as *const libc::c_char,
            path,
        );
    }
    libc::rewind(file);
    let mut buffer: *mut libc::c_char = MemAlloc(
        (::core::mem::size_of::<libc::c_char>())
            .wrapping_mul((size as usize).wrapping_add(1 as usize)),
    ) as *mut libc::c_char;
    let mut result: usize =
        libc::fread(buffer as *mut libc::c_void, size as usize, 1 as usize, file);
    if result != 1 as usize {
        Fatal(
            b"File_Read: failed to read correct number of bytes from '%s'\0" as *const u8
                as *const libc::c_char,
            path,
        );
    }
    libc::fclose(file);
    *buffer.offset(size as isize) = 0 as i32 as libc::c_char;
    return buffer as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn File_Size(mut path: *const libc::c_char) -> i64 {
    let mut file: *mut libc::FILE = libc::fopen(path, b"rb\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        return 0 as i32 as i64;
    }
    libc::fseek(file, 0 as libc::c_long, 2 as i32);
    let mut size: i64 = libc::ftell(file) as i64;
    libc::fclose(file);
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn File_Read(mut this: *mut File, mut data: *mut libc::c_void, mut len: u32) {
    libc::fread(data, len as usize, 1 as usize, (*this).handle);
}
#[no_mangle]
pub unsafe extern "C" fn File_Write(
    mut this: *mut File,
    mut data: *const libc::c_void,
    mut len: u32,
) {
    libc::fwrite(data, len as usize, 1 as usize, (*this).handle);
}
#[no_mangle]
pub unsafe extern "C" fn File_WriteStr(mut this: *mut File, mut data: *const libc::c_char) {
    libc::fwrite(
        data as *const libc::c_void,
        StrLen(data),
        1 as usize,
        (*this).handle,
    );
}
#[no_mangle]
pub unsafe extern "C" fn File_ReadI64(mut this: *mut File) -> i64 {
    let mut value: i64 = 0;
    libc::fread(
        &mut value as *mut i64 as *mut libc::c_void,
        ::core::mem::size_of::<i64>() as usize,
        1 as usize,
        (*this).handle,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn File_WriteU64(mut this: *mut File, mut value: u64) {
    libc::fwrite(
        &mut value as *mut u64 as *const libc::c_void,
        ::core::mem::size_of::<u64>() as usize,
        1 as usize,
        (*this).handle,
    );
}
#[no_mangle]
pub unsafe extern "C" fn File_WriteI8(mut this: *mut File, mut value: i8) {
    libc::fwrite(
        &mut value as *mut i8 as *const libc::c_void,
        ::core::mem::size_of::<i8>() as usize,
        1 as usize,
        (*this).handle,
    );
}
#[no_mangle]
pub unsafe extern "C" fn File_WriteU32(mut this: *mut File, mut value: u32) {
    libc::fwrite(
        &mut value as *mut u32 as *const libc::c_void,
        ::core::mem::size_of::<u32>() as usize,
        1 as usize,
        (*this).handle,
    );
}
#[no_mangle]
pub unsafe extern "C" fn File_WriteI16(mut this: *mut File, mut value: i16) {
    libc::fwrite(
        &mut value as *mut i16 as *const libc::c_void,
        ::core::mem::size_of::<i16>() as usize,
        1 as usize,
        (*this).handle,
    );
}
#[no_mangle]
pub unsafe extern "C" fn File_ReadU64(mut this: *mut File) -> u64 {
    let mut value: u64 = 0;
    libc::fread(
        &mut value as *mut u64 as *mut libc::c_void,
        ::core::mem::size_of::<u64>() as usize,
        1 as usize,
        (*this).handle,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn File_ReadI32(mut this: *mut File) -> i32 {
    let mut value: i32 = 0;
    libc::fread(
        &mut value as *mut i32 as *mut libc::c_void,
        ::core::mem::size_of::<i32>() as usize,
        1 as usize,
        (*this).handle,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn File_ReadU32(mut this: *mut File) -> u32 {
    let mut value: u32 = 0;
    libc::fread(
        &mut value as *mut u32 as *mut libc::c_void,
        ::core::mem::size_of::<u32>() as usize,
        1 as usize,
        (*this).handle,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn File_WriteU16(mut this: *mut File, mut value: u16) {
    libc::fwrite(
        &mut value as *mut u16 as *const libc::c_void,
        ::core::mem::size_of::<u16>() as usize,
        1 as usize,
        (*this).handle,
    );
}
#[no_mangle]
pub unsafe extern "C" fn File_ReadU8(mut this: *mut File) -> u8 {
    let mut value: u8 = 0;
    libc::fread(
        &mut value as *mut u8 as *mut libc::c_void,
        ::core::mem::size_of::<u8>() as usize,
        1 as usize,
        (*this).handle,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn File_ReadU16(mut this: *mut File) -> u16 {
    let mut value: u16 = 0;
    libc::fread(
        &mut value as *mut u16 as *mut libc::c_void,
        ::core::mem::size_of::<u16>() as usize,
        1 as usize,
        (*this).handle,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn File_ReadI16(mut this: *mut File) -> i16 {
    let mut value: i16 = 0;
    libc::fread(
        &mut value as *mut i16 as *mut libc::c_void,
        ::core::mem::size_of::<i16>() as usize,
        1 as usize,
        (*this).handle,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn File_ReadF32(mut this: *mut File) -> f32 {
    let mut value: f32 = 0.;
    libc::fread(
        &mut value as *mut f32 as *mut libc::c_void,
        ::core::mem::size_of::<f32>() as usize,
        1 as usize,
        (*this).handle,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn File_ReadF64(mut this: *mut File) -> f64 {
    let mut value: f64 = 0.;
    libc::fread(
        &mut value as *mut f64 as *mut libc::c_void,
        ::core::mem::size_of::<f64>() as usize,
        1 as usize,
        (*this).handle,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn File_WriteU8(mut this: *mut File, mut value: u8) {
    libc::fwrite(
        &mut value as *mut u8 as *const libc::c_void,
        ::core::mem::size_of::<u8>() as usize,
        1 as usize,
        (*this).handle,
    );
}
#[no_mangle]
pub unsafe extern "C" fn File_WriteI32(mut this: *mut File, mut value: i32) {
    libc::fwrite(
        &mut value as *mut i32 as *const libc::c_void,
        ::core::mem::size_of::<i32>() as usize,
        1 as usize,
        (*this).handle,
    );
}
#[no_mangle]
pub unsafe extern "C" fn File_WriteI64(mut this: *mut File, mut value: i64) {
    libc::fwrite(
        &mut value as *mut i64 as *const libc::c_void,
        ::core::mem::size_of::<i64>() as usize,
        1 as usize,
        (*this).handle,
    );
}
#[no_mangle]
pub unsafe extern "C" fn File_WriteF32(mut this: *mut File, mut value: f32) {
    libc::fwrite(
        &mut value as *mut f32 as *const libc::c_void,
        ::core::mem::size_of::<f32>() as usize,
        1 as usize,
        (*this).handle,
    );
}
#[no_mangle]
pub unsafe extern "C" fn File_WriteF64(mut this: *mut File, mut value: f64) {
    libc::fwrite(
        &mut value as *mut f64 as *const libc::c_void,
        ::core::mem::size_of::<f64>() as usize,
        1 as usize,
        (*this).handle,
    );
}
#[no_mangle]
pub unsafe extern "C" fn File_ReadI8(mut this: *mut File) -> i8 {
    let mut value: i8 = 0;
    libc::fread(
        &mut value as *mut i8 as *mut libc::c_void,
        ::core::mem::size_of::<i8>() as usize,
        1 as usize,
        (*this).handle,
    );
    return value;
}
