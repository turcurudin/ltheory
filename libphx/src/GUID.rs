use ::libc;
use glam::Vec3;
use crate::internal::Memory::*;
#[no_mangle]
pub static mut nextID: u64 = 1 as libc::c_int as u64;
#[no_mangle]
pub unsafe extern "C" fn GUID_Create() -> u64 {
    let fresh0 = nextID;
    nextID = nextID.wrapping_add(1);
    return fresh0;
}
#[no_mangle]
pub unsafe extern "C" fn GUID_Exists(mut id: u64) -> bool {
    return id < nextID && id != 0 as libc::c_ulonglong;
}
#[no_mangle]
pub unsafe extern "C" fn GUID_Reset() {
    nextID = 1 as libc::c_int as u64;
}
