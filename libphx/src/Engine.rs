use crate::internal::Memory::*;
use crate::PhxSignal::*;
use crate::ResourceType::*;
use glam::Vec3;
use libc;
use sdl2_sys::*;

extern "C" {
    fn Directory_Create(path: *const libc::c_char) -> bool;
    fn Fatal(_: *const libc::c_char, _: ...);
    fn Gamepad_Update();
    fn Input_Init();
    fn Input_Free();
    fn Input_Update();
    fn Joystick_Update();
    fn Keyboard_Init();
    fn Keyboard_Free();
    fn Keyboard_UpdatePre();
    fn Keyboard_UpdatePost();
    fn Metric_Reset();
    fn Mouse_Init();
    fn Mouse_Free();
    fn Mouse_Update();
    fn Signal_Init();
    fn Signal_Free();
    fn Profiler_Begin(_: *const libc::c_char);
    fn Profiler_End();
    fn Resource_Init();
    fn exit(_: i32) -> !;
    fn abort() -> !;
    fn puts(_: *const libc::c_char) -> i32;
    fn printf(_: *const libc::c_char, _: ...) -> i32;
    fn atexit(_: Option<unsafe extern "C" fn() -> ()>) -> i32;
    fn ShaderVar_Init();
    fn ShaderVar_Free();
    fn TimeStamp_Get() -> TimeStamp;
    fn TimeStamp_GetElapsed(start: TimeStamp) -> f64;
}
pub type ResourceType = i32;
pub type TimeStamp = u64;

#[no_mangle]
pub static mut subsystems: u32 =
    SDL_INIT_EVENTS |
    SDL_INIT_VIDEO |
    SDL_INIT_TIMER |
    SDL_INIT_HAPTIC |
    SDL_INIT_JOYSTICK |
    SDL_INIT_GAMECONTROLLER;

static mut initTime: TimeStamp = 0 as i32 as TimeStamp;

#[no_mangle]
pub unsafe extern "C" fn Engine_Init(mut glVersionMajor: i32, mut glVersionMinor: i32) {
    static mut firstTime: bool = 1 as i32 != 0;
    Signal_Init();
    printf(
        b"Engine_Init: Requesting GL %d.%d\n\0" as *const u8 as *const libc::c_char,
        glVersionMajor,
        glVersionMinor,
    );
    if firstTime {
        firstTime = 0 as i32 != 0;
        let mut compiled: SDL_version = SDL_version {
            major: 0,
            minor: 0,
            patch: 0,
        };
        let mut linked: SDL_version = SDL_version {
            major: 0,
            minor: 0,
            patch: 0,
        };
        compiled.major = 2;
        compiled.minor = 26;
        compiled.patch = 1;
        SDL_GetVersion(&mut linked);
        if compiled.major != linked.major {
            puts(
                b"Engine_Init: Detected SDL major version mismatch:\0" as *const u8
                    as *const libc::c_char,
            );
            printf(
                b"  Version (Compiled) : %d.%d.%d\n\0" as *const u8 as *const libc::c_char,
                compiled.major as i32,
                compiled.minor as i32,
                compiled.patch as i32,
            );
            printf(
                b"  Version (Linked)   : %d.%d.%d\n\0" as *const u8 as *const libc::c_char,
                linked.major as i32,
                linked.minor as i32,
                linked.patch as i32,
            );
            Fatal(b"Engine_Init: Terminating.\0" as *const u8 as *const libc::c_char);
        }
        if SDL_Init(0 as i32 as u32) != 0 {
            Fatal(b"Engine_Init: Failed to initialize SDL\0" as *const u8 as *const libc::c_char);
        }
        if !Directory_Create(b"log\0" as *const u8 as *const libc::c_char) {
            Fatal(
                b"Engine_Init: Failed to create log directory.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        atexit(Some(SDL_Quit as unsafe extern "C" fn() -> ()));
    }
    if SDL_InitSubSystem(subsystems) != 0 {
        Fatal(
            b"Engine_Init: Failed to initialize SDL's subsystems\0" as *const u8
                as *const libc::c_char,
        );
    }
    SDL_GL_SetAttribute(SDL_GLattr::SDL_GL_CONTEXT_MAJOR_VERSION, glVersionMajor);
    SDL_GL_SetAttribute(SDL_GLattr::SDL_GL_CONTEXT_MINOR_VERSION, glVersionMinor);
    SDL_GL_SetAttribute(
        SDL_GLattr::SDL_GL_CONTEXT_PROFILE_MASK,
        SDL_GLprofile::SDL_GL_CONTEXT_PROFILE_COMPATIBILITY as i32,
    );
    SDL_GL_SetAttribute(SDL_GLattr::SDL_GL_ACCELERATED_VISUAL, 1);
    SDL_GL_SetAttribute(SDL_GLattr::SDL_GL_RED_SIZE, 8);
    SDL_GL_SetAttribute(SDL_GLattr::SDL_GL_GREEN_SIZE, 8);
    SDL_GL_SetAttribute(SDL_GLattr::SDL_GL_BLUE_SIZE, 8);
    SDL_GL_SetAttribute(SDL_GLattr::SDL_GL_DOUBLEBUFFER, 1);
    SDL_GL_SetAttribute(SDL_GLattr::SDL_GL_DEPTH_SIZE, 24);
    Keyboard_Init();
    Metric_Reset();
    Mouse_Init();
    Input_Init();
    Resource_Init();
    ShaderVar_Init();
    initTime = TimeStamp_Get();
}
#[no_mangle]
pub unsafe extern "C" fn Engine_Free() {
    ShaderVar_Free();
    Keyboard_Free();
    Mouse_Free();
    Input_Free();
    Signal_Free();
    SDL_QuitSubSystem(subsystems);
}
#[no_mangle]
pub unsafe extern "C" fn Engine_Abort() {
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn Engine_GetBits() -> i32 {
    return (8 as usize).wrapping_mul(::core::mem::size_of::<*mut libc::c_void>()) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn Engine_GetTime() -> f64 {
    return TimeStamp_GetElapsed(initTime);
}
#[no_mangle]
pub unsafe extern "C" fn Engine_GetVersion() -> *const libc::c_char {
    return env!("PHX_VERSION").as_ptr() as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn Engine_IsInitialized() -> bool {
    return initTime != 0 as u64;
}
#[no_mangle]
pub unsafe extern "C" fn Engine_Terminate() {
    exit(0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn Engine_Update() {
    Profiler_Begin(
        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"Engine_Update\0")).as_ptr(),
    );
    Metric_Reset();
    Keyboard_UpdatePre();
    Mouse_Update();
    Joystick_Update();
    Gamepad_Update();
    Input_Update();
    Keyboard_UpdatePost();
    Profiler_End();
}
