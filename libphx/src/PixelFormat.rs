use ::libc;
use glam::Vec3;
use crate::internal::Memory::*;
pub type PixelFormat = i32;
#[no_mangle]
pub static mut PixelFormat_Red: PixelFormat = 0x1903 as libc::c_int;
#[no_mangle]
pub static mut PixelFormat_RG: PixelFormat = 0x8227 as libc::c_int;
#[no_mangle]
pub static mut PixelFormat_RGB: PixelFormat = 0x1907 as libc::c_int;
#[no_mangle]
pub static mut PixelFormat_BGR: PixelFormat = 0x80e0 as libc::c_int;
#[no_mangle]
pub static mut PixelFormat_RGBA: PixelFormat = 0x1908 as libc::c_int;
#[no_mangle]
pub static mut PixelFormat_BGRA: PixelFormat = 0x80e1 as libc::c_int;
#[no_mangle]
pub static mut PixelFormat_Depth_Component: PixelFormat = 0x1902 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn PixelFormat_Components(mut this: PixelFormat) -> libc::c_int {
    match this {
        6403 | 6402 => return 1 as libc::c_int,
        33319 => return 2 as libc::c_int,
        6407 | 32992 => return 3 as libc::c_int,
        6408 | 32993 => return 4 as libc::c_int,
        _ => {}
    }
    return 0 as libc::c_int;
}
