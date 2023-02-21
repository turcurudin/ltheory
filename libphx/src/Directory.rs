use ::libc;
use glam::Vec3;
use crate::internal::Memory::*;
extern "C" {
    pub type _telldir;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> i32;
    fn File_IsDir(path: cstr) -> bool;
    fn closedir(_: *mut DIR) -> i32;
    fn opendir(_: *const libc::c_char) -> *mut DIR;
    fn readdir(_: *mut DIR) -> *mut dirent;
    fn chdir(_: *const libc::c_char) -> i32;
    fn getcwd(_: *mut libc::c_char, _: usize) -> *mut libc::c_char;
    fn rmdir(_: *const libc::c_char) -> i32;
    fn mkdir(_: *const libc::c_char, _: mode_t) -> i32;
}
pub type __u8_t = libc::c_uchar;
pub type __u16 = u16;
pub type __u64_t = u64;
pub type __darwin_mode_t = __u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: libc::c_long,
    pub __opaque: [libc::c_char; 56],
}
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type cstr = *const libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Directory {
    pub handle: *mut DIR,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DIR {
    pub __dd_fd: i32,
    pub __dd_loc: libc::c_long,
    pub __dd_size: libc::c_long,
    pub __dd_buf: *mut libc::c_char,
    pub __dd_len: i32,
    pub __dd_seek: libc::c_long,
    pub __padding: libc::c_long,
    pub __dd_flags: i32,
    pub __dd_lock: __darwin_pthread_mutex_t,
    pub __dd_td: *mut _telldir,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __u64_t,
    pub d_seekoff: __u64_t,
    pub d_reclen: __u16,
    pub d_namlen: __u16,
    pub d_type: __u8_t,
    pub d_name: [libc::c_char; 1024],
}
pub type mode_t = __darwin_mode_t;

#[no_mangle]
pub unsafe extern "C" fn Directory_Open(mut path: cstr) -> *mut Directory {
    let mut dir: *mut DIR = opendir(path);
    if dir.is_null() {
        return 0 as *mut Directory;
    }
    let mut this: *mut Directory = MemAlloc(
        ::core::mem::size_of::<Directory>() as usize,
    ) as *mut Directory;
    (*this).handle = dir;
    return this;
}
#[no_mangle]
pub unsafe extern "C" fn Directory_Close(mut this: *mut Directory) {
    closedir((*this).handle);
    MemFree(this as *const libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn Directory_GetNext(mut this: *mut Directory) -> cstr {
    loop {
        let mut ent: *mut dirent = readdir((*this).handle);
        if ent.is_null() {
            return 0 as cstr;
        }
        if StrEqual(
            ((*ent).d_name).as_mut_ptr() as cstr,
            b".\0" as *const u8 as *const libc::c_char,
        ) as i32 != 0
            || StrEqual(
                ((*ent).d_name).as_mut_ptr() as cstr,
                b"..\0" as *const u8 as *const libc::c_char,
            ) as i32 != 0
        {
            continue;
        }
        return ((*ent).d_name).as_mut_ptr() as cstr;
    };
}
#[no_mangle]
pub unsafe extern "C" fn Directory_Change(mut cwd: cstr) -> bool {
    return chdir(cwd) == 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn Directory_Create(mut path: cstr) -> bool {
    mkdir(path, 0o775 as i32 as mode_t);
    return File_IsDir(path);
}
#[no_mangle]
pub unsafe extern "C" fn Directory_GetCurrent() -> cstr {
    static mut buffer: [libc::c_char; 1024] = [0; 1024];
    if !(getcwd(
        buffer.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as usize,
    ))
        .is_null()
    {
        return 0 as cstr;
    }
    buffer[(::core::mem::size_of::<[libc::c_char; 1024]>())
        .wrapping_sub(1 as usize)] = 0 as i32 as libc::c_char;
    return buffer.as_mut_ptr() as cstr;
}
#[no_mangle]
pub unsafe extern "C" fn Directory_Remove(mut path: cstr) -> bool {
    return rmdir(path) == 0 as i32;
}
