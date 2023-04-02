use crate::internal::Memory::*;
use crate::Common::*;
use crate::HatDir::*;
use crate::Math::Vec3;
use crate::TimeStamp::*;
use libc;
use sdl2_sys::*;

extern "C" {
    pub type _SDL_Joystick;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Joystick {
    pub handle: *mut SDL_Joystick,
    pub guid: *const libc::c_char,
    pub axes: i32,
    pub balls: i32,
    pub buttons: i32,
    pub hats: i32,
    pub buttonStates: *mut bool,
    pub axisAlive: *mut bool,
    pub axisStates: *mut f64,
    pub lastUsed: TimeStamp,
}

static mut kMaxOpen: i32 = 64;

static mut kOpen: i32 = 0;

static mut freeList: [*mut Joystick; 64] = [
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
];

unsafe extern "C" fn ConvertGUID(id: SDL_JoystickGUID) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 64] = [0; 64];
    SDL_JoystickGetGUIDString(
        id,
        buf.as_mut_ptr(),
        std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    buf.as_mut_ptr()
}

unsafe extern "C" fn Joystick_UpdateSingle(this: *mut Joystick) {
    let mut changed: bool = false;
    let mut i: i32 = 0;
    while i < (*this).axes {
        let state: f64 = Joystick_GetAxis(this, i);
        let delta: f64 = f64::abs(state - *((*this).axisStates).offset(i as isize));
        if delta > 0.1f64 {
            changed = true;
            *((*this).axisAlive).offset(i as isize) = true;
        }
        *((*this).axisStates).offset(i as isize) = state;
        i += 1;
    }
    let mut i_0: i32 = 0;
    while i_0 < (*this).buttons {
        let state_0: bool = Joystick_ButtonDown(this, i_0);
        if *((*this).buttonStates).offset(i_0 as isize) as i32 != state_0 as i32 {
            changed = true;
        }
        *((*this).buttonStates).offset(i_0 as isize) = state_0;
        i_0 += 1;
    }
    if changed {
        (*this).lastUsed = TimeStamp_Get();
    }
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_GetCount() -> i32 {
    SDL_NumJoysticks()
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_Open(index: i32) -> *mut Joystick {
    let this = MemNew!(Joystick);
    if kOpen == kMaxOpen {
        CFatal!("Cannot open any more gamepad connections.");
    }
    let mut i: i32 = 0;
    while i < kMaxOpen {
        if (freeList[i as usize]).is_null() {
            freeList[i as usize] = this;
            kOpen += 1;
            break;
        } else {
            i += 1;
        }
    }
    (*this).handle = SDL_JoystickOpen(index);
    (*this).guid = StrDup(ConvertGUID(SDL_JoystickGetGUID((*this).handle)) as *const libc::c_char);
    (*this).axes = SDL_JoystickNumAxes((*this).handle);
    (*this).balls = SDL_JoystickNumBalls((*this).handle);
    (*this).buttons = SDL_JoystickNumButtons((*this).handle);
    (*this).hats = SDL_JoystickNumHats((*this).handle);
    (*this).buttonStates = MemNewArray!(bool, (*this).buttons);
    (*this).axisAlive = MemNewArray!(bool, (*this).axes);
    MemZero(
        (*this).axisAlive as *mut _,
        (std::mem::size_of::<bool>()).wrapping_mul((*this).axes as usize),
    );
    (*this).axisStates = MemNewArray!(f64, (*this).axes);
    (*this).lastUsed = TimeStamp_Get();
    Joystick_UpdateSingle(this);
    this
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_Close(this: *mut Joystick) {
    kOpen -= 1;
    let mut i: i32 = 0;
    while i < kMaxOpen {
        if freeList[i as usize] == this {
            freeList[i as usize] = std::ptr::null_mut();
            break;
        } else {
            i += 1;
        }
    }
    SDL_JoystickClose((*this).handle);
    MemFree((*this).guid as *const _);
    MemFree((*this).buttonStates as *const _);
    MemFree((*this).axisStates as *const _);
    MemFree(this as *const _);
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_GetGUID(this: *mut Joystick) -> *const libc::c_char {
    (*this).guid
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_GetGUIDByIndex(index: i32) -> *const libc::c_char {
    ConvertGUID(SDL_JoystickGetDeviceGUID(index)) as *const libc::c_char
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_GetName(this: *mut Joystick) -> *const libc::c_char {
    SDL_JoystickName((*this).handle)
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_GetNameByIndex(index: i32) -> *const libc::c_char {
    SDL_JoystickNameForIndex(index)
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_GetAxisCount(this: *mut Joystick) -> i32 {
    (*this).axes
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_GetBallCount(this: *mut Joystick) -> i32 {
    (*this).balls
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_GetButtonCount(this: *mut Joystick) -> i32 {
    (*this).buttons
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_GetHatCount(this: *mut Joystick) -> i32 {
    (*this).hats
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_GetIdleTime(this: *mut Joystick) -> f64 {
    TimeStamp_GetElapsed((*this).lastUsed)
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_GetAxis(this: *mut Joystick, index: i32) -> f64 {
    SDL_JoystickGetAxis((*this).handle, index) as i32 as f64 / 32768.0f64
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_GetAxisAlive(this: *mut Joystick, index: i32) -> bool {
    *((*this).axisAlive).offset(index as isize)
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_GetAxisDelta(this: *mut Joystick, index: i32) -> f64 {
    SDL_JoystickGetAxis((*this).handle, index) as i32 as f64 / 32768.0f64
        - *((*this).axisStates).offset(index as isize)
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_GetHat(this: *mut Joystick, index: i32) -> HatDir {
    SDL_JoystickGetHat((*this).handle, index) as HatDir
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_ButtonDown(this: *mut Joystick, index: i32) -> bool {
    SDL_JoystickGetButton((*this).handle, index) as i32 > 0
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_ButtonPressed(this: *mut Joystick, index: i32) -> bool {
    SDL_JoystickGetButton((*this).handle, index) as i32 > 0
        && !*((*this).buttonStates).offset(index as isize)
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_ButtonReleased(this: *mut Joystick, index: i32) -> bool {
    SDL_JoystickGetButton((*this).handle, index) as i32 == 0
        && *((*this).buttonStates).offset(index as isize) as i32 != 0
}

#[no_mangle]
pub unsafe extern "C" fn Joystick_Update() {
    let mut i: i32 = 0;
    while i < kMaxOpen {
        if !(freeList[i as usize]).is_null() {
            Joystick_UpdateSingle(freeList[i as usize]);
        }
        i += 1;
    }
}
