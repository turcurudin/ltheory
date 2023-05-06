
use crate::phx::Common::*;
use crate::phx::Directory::*;
use crate::phx::Gamepad::*;
use crate::phx::Input::*;
use crate::phx::Joystick::*;
use crate::phx::Keyboard::*;

use crate::phx::Metric::*;
use crate::phx::Mouse::*;
use crate::phx::Profiler::*;
use crate::phx::Resource::*;

use crate::phx::ShaderVar::*;
use crate::phx::Signal::*;
use crate::phx::TimeStamp::*;
use sdl2_sys::*;

// /* On Windows, request usage of the dedicated GPU if the machine switches
//  * between on-board and dedicated GPUs dynamically. Only works when exported
//  * by the exe, not when exported by a dll. */
//  #if WINDOWS
//  extern "C" {
//    __declspec(dllexport) ulong NvOptimusEnablement = 0x00000001;
//    __declspec(dllexport) int AmdPowerXpressRequestHighPerformance = 1;
//  }
// #endif

#[no_mangle]
pub static subsystems: u32 = SDL_INIT_EVENTS
    | SDL_INIT_VIDEO
    | SDL_INIT_TIMER
    | SDL_INIT_HAPTIC
    | SDL_INIT_JOYSTICK
    | SDL_INIT_GAMECONTROLLER;

static mut initTime: TimeStamp = 0;

#[no_mangle]
pub unsafe extern "C" fn Engine_Init(glVersionMajor: i32, glVersionMinor: i32) {
    static mut firstTime: bool = true;
    Signal_Init();

    CPrintf!(
        "Engine_Init: Requesting GL %d.%d\n",
        glVersionMajor,
        glVersionMinor,
    );

    if firstTime {
        firstTime = false;

        /* Check SDL version compatibility. */
        let compiled: SDL_version = SDL_version {
            major: SDL_MAJOR_VERSION as u8,
            minor: SDL_MINOR_VERSION as u8,
            patch: SDL_PATCHLEVEL as u8,
        };
        let mut linked: SDL_version = SDL_version {
            major: 0,
            minor: 0,
            patch: 0,
        };
        SDL_GetVersion(&mut linked);
        if compiled.major != linked.major {
            println!("Engine_Init: Detected SDL major version mismatch:");
            CPrintf!(
                "  Version (Compiled) : %d.%d.%d\n",
                compiled.major as i32,
                compiled.minor as i32,
                compiled.patch as i32,
            );
            CPrintf!(
                "  Version (Linked)   : %d.%d.%d\n",
                linked.major as i32,
                linked.minor as i32,
                linked.patch as i32,
            );
            CFatal!("Engine_Init: Terminating.");
        }

        if SDL_Init(0) != 0 {
            CFatal!("Engine_Init: Failed to initialize SDL");
        }
        if !Directory_Create(c_str!("log")) {
            CFatal!("Engine_Init: Failed to create log directory.");
        }
        atexit(Some(SDL_Quit as unsafe extern "C" fn() -> ()));
    }

    if SDL_InitSubSystem(subsystems) != 0 {
        CFatal!("Engine_Init: Failed to initialize SDL's subsystems");
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
pub extern "C" fn Engine_GetBits() -> i32 {
    8_usize.wrapping_mul(std::mem::size_of::<*mut libc::c_void>()) as i32
}

#[no_mangle]
pub unsafe extern "C" fn Engine_GetTime() -> f64 {
    TimeStamp_GetElapsed(initTime)
}

#[no_mangle]
pub extern "C" fn Engine_GetVersion() -> *const libc::c_char {
    env!("PHX_VERSION").as_ptr() as *const libc::c_char
}

#[no_mangle]
pub unsafe extern "C" fn Engine_IsInitialized() -> bool {
    initTime != 0
}

#[no_mangle]
pub unsafe extern "C" fn Engine_Terminate() {
    exit(0);
}

#[no_mangle]
pub unsafe extern "C" fn Engine_Update() {
    Profiler_Begin(c_str!("Engine_Update"));
    Metric_Reset();
    Keyboard_UpdatePre();
    Mouse_Update();
    Joystick_Update();
    Gamepad_Update();
    Input_Update();
    Keyboard_UpdatePost();
    Profiler_End();
}
