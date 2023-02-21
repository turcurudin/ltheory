use ::libc;
use glam::Vec3;
use crate::internal::Memory::*;
pub type GamepadButton = i32;
pub const SDL_CONTROLLER_BUTTON_A: C2RustUnnamed = 0;
pub const SDL_CONTROLLER_BUTTON_B: C2RustUnnamed = 1;
pub const SDL_CONTROLLER_BUTTON_X: C2RustUnnamed = 2;
pub const SDL_CONTROLLER_BUTTON_Y: C2RustUnnamed = 3;
pub const SDL_CONTROLLER_BUTTON_BACK: C2RustUnnamed = 4;
pub const SDL_CONTROLLER_BUTTON_GUIDE: C2RustUnnamed = 5;
pub const SDL_CONTROLLER_BUTTON_START: C2RustUnnamed = 6;
pub const SDL_CONTROLLER_BUTTON_LEFTSTICK: C2RustUnnamed = 7;
pub const SDL_CONTROLLER_BUTTON_RIGHTSTICK: C2RustUnnamed = 8;
pub const SDL_CONTROLLER_BUTTON_LEFTSHOULDER: C2RustUnnamed = 9;
pub const SDL_CONTROLLER_BUTTON_RIGHTSHOULDER: C2RustUnnamed = 10;
pub const SDL_CONTROLLER_BUTTON_DPAD_UP: C2RustUnnamed = 11;
pub const SDL_CONTROLLER_BUTTON_DPAD_DOWN: C2RustUnnamed = 12;
pub const SDL_CONTROLLER_BUTTON_DPAD_LEFT: C2RustUnnamed = 13;
pub const SDL_CONTROLLER_BUTTON_DPAD_RIGHT: C2RustUnnamed = 14;
pub type C2RustUnnamed = i32;
pub const SDL_CONTROLLER_BUTTON_MAX: C2RustUnnamed = 21;
pub const SDL_CONTROLLER_BUTTON_TOUCHPAD: C2RustUnnamed = 20;
pub const SDL_CONTROLLER_BUTTON_PADDLE4: C2RustUnnamed = 19;
pub const SDL_CONTROLLER_BUTTON_PADDLE3: C2RustUnnamed = 18;
pub const SDL_CONTROLLER_BUTTON_PADDLE2: C2RustUnnamed = 17;
pub const SDL_CONTROLLER_BUTTON_PADDLE1: C2RustUnnamed = 16;
pub const SDL_CONTROLLER_BUTTON_MISC1: C2RustUnnamed = 15;
pub const SDL_CONTROLLER_BUTTON_INVALID: C2RustUnnamed = -1;
#[no_mangle]
pub static mut GamepadButton_BEGIN: GamepadButton = SDL_CONTROLLER_BUTTON_A
    as i32;
#[no_mangle]
pub static mut GamepadButton_A: GamepadButton = SDL_CONTROLLER_BUTTON_A as i32;
#[no_mangle]
pub static mut GamepadButton_B: GamepadButton = SDL_CONTROLLER_BUTTON_B as i32;
#[no_mangle]
pub static mut GamepadButton_X: GamepadButton = SDL_CONTROLLER_BUTTON_X as i32;
#[no_mangle]
pub static mut GamepadButton_Y: GamepadButton = SDL_CONTROLLER_BUTTON_Y as i32;
#[no_mangle]
pub static mut GamepadButton_Back: GamepadButton = SDL_CONTROLLER_BUTTON_BACK
    as i32;
#[no_mangle]
pub static mut GamepadButton_Guide: GamepadButton = SDL_CONTROLLER_BUTTON_GUIDE
    as i32;
#[no_mangle]
pub static mut GamepadButton_Start: GamepadButton = SDL_CONTROLLER_BUTTON_START
    as i32;
#[no_mangle]
pub static mut GamepadButton_LStick: GamepadButton = SDL_CONTROLLER_BUTTON_LEFTSTICK
    as i32;
#[no_mangle]
pub static mut GamepadButton_RStick: GamepadButton = SDL_CONTROLLER_BUTTON_RIGHTSTICK
    as i32;
#[no_mangle]
pub static mut GamepadButton_LBumper: GamepadButton = SDL_CONTROLLER_BUTTON_LEFTSHOULDER
    as i32;
#[no_mangle]
pub static mut GamepadButton_RBumper: GamepadButton = SDL_CONTROLLER_BUTTON_RIGHTSHOULDER
    as i32;
#[no_mangle]
pub static mut GamepadButton_Up: GamepadButton = SDL_CONTROLLER_BUTTON_DPAD_UP
    as i32;
#[no_mangle]
pub static mut GamepadButton_Down: GamepadButton = SDL_CONTROLLER_BUTTON_DPAD_DOWN
    as i32;
#[no_mangle]
pub static mut GamepadButton_Left: GamepadButton = SDL_CONTROLLER_BUTTON_DPAD_LEFT
    as i32;
#[no_mangle]
pub static mut GamepadButton_Right: GamepadButton = SDL_CONTROLLER_BUTTON_DPAD_RIGHT
    as i32;
#[no_mangle]
pub static mut GamepadButton_END: GamepadButton = SDL_CONTROLLER_BUTTON_DPAD_RIGHT
    as i32;
