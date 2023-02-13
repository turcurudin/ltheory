use ::libc;
use crate::internal::Memory::*;
extern "C" {
    pub type MemPool;
    pub type FMOD_CHANNEL;
    pub type FMOD_SOUND;
    pub type StrMap;
    pub type FMOD_SYSTEM;
    fn Fatal(_: cstr, _: ...);
    fn FMOD_Debug_Initialize(
        flags: FMOD_DEBUG_FLAGS,
        mode: FMOD_DEBUG_MODE,
        callback: FMOD_DEBUG_CALLBACK,
        filename: *const libc::c_char,
    ) -> FMOD_RESULT;
    fn FMOD_System_Create(
        system: *mut *mut FMOD_SYSTEM,
        headerversion: libc::c_uint,
    ) -> FMOD_RESULT;
    fn FMOD_System_Release(system: *mut FMOD_SYSTEM) -> FMOD_RESULT;
    fn FMOD_System_Init(
        system: *mut FMOD_SYSTEM,
        maxchannels: libc::c_int,
        flags: FMOD_INITFLAGS,
        extradriverdata: *mut libc::c_void,
    ) -> FMOD_RESULT;
    fn FMOD_System_Update(system: *mut FMOD_SYSTEM) -> FMOD_RESULT;
    fn FMOD_System_Set3DSettings(
        system: *mut FMOD_SYSTEM,
        dopplerscale: libc::c_float,
        distancefactor: libc::c_float,
        rolloffscale: libc::c_float,
    ) -> FMOD_RESULT;
    fn FMOD_System_Set3DListenerAttributes(
        system: *mut FMOD_SYSTEM,
        listener: libc::c_int,
        pos: *const FMOD_VECTOR,
        vel: *const FMOD_VECTOR,
        forward: *const FMOD_VECTOR,
        up: *const FMOD_VECTOR,
    ) -> FMOD_RESULT;
    fn FMOD_System_GetVersion(
        system: *mut FMOD_SYSTEM,
        version: *mut libc::c_uint,
    ) -> FMOD_RESULT;
    fn MemPool_Create(cellSize: uint32, blockSize: uint32) -> *mut MemPool;
    fn MemPool_Free(_: *mut MemPool);
    fn MemPool_Alloc(_: *mut MemPool) -> *mut libc::c_void;
    fn MemPool_Dealloc(_: *mut MemPool, _: *mut libc::c_void);
    fn MemPool_GetSize(_: *mut MemPool) -> uint32;
    fn Sound_IsPlaying(_: *mut Sound) -> bool;
    fn Sound_Update(_: *mut Sound);
    fn Sound_IsFreed(_: *mut Sound) -> bool;
    fn StrMap_Create(initCapacity: uint32) -> *mut StrMap;
    fn StrMap_Free(_: *mut StrMap);
    fn StrMap_Get(_: *mut StrMap, key: cstr) -> *mut libc::c_void;
    fn StrMap_GetSize(_: *mut StrMap) -> uint32;
    fn StrMap_Remove(_: *mut StrMap, key: cstr);
    fn StrMap_Set(_: *mut StrMap, key: cstr, val: *mut libc::c_void);
}
pub type int32_t = libc::c_int;
pub type uint8_t = libc::c_uchar;
pub type uint32_t = libc::c_uint;
pub type uint = libc::c_uint;
pub type cstr = *const libc::c_char;
pub type int32 = int32_t;
pub type uint8 = uint8_t;
pub type uint32 = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sound {
    pub desc: *mut SoundDesc,
    pub handle: *mut FMOD_CHANNEL,
    pub state: SoundState,
    pub autoPos: *const Vec3f,
    pub autoVel: *const Vec3f,
    pub freeOnFinish: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec3f {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
}
pub type SoundState = uint8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SoundDesc {
    pub _refCount: uint32,
    pub handle: *mut FMOD_SOUND,
    pub name: cstr,
    pub path: cstr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Audio {
    pub handle: *mut FMOD_SYSTEM,
    pub descMap: *mut StrMap,
    pub soundPool: *mut MemPool,
    pub playingSounds_size: int32,
    pub playingSounds_capacity: int32,
    pub playingSounds_data: *mut *mut Sound,
    pub freeingSounds_size: int32,
    pub freeingSounds_capacity: int32,
    pub freeingSounds_data: *mut *mut Sound,
    pub autoPos: *const Vec3f,
    pub autoVel: *const Vec3f,
    pub autoFwd: *const Vec3f,
    pub autoUp: *const Vec3f,
}
pub type FMOD_RESULT = libc::c_uint;
pub const FMOD_RESULT_FORCEINT: FMOD_RESULT = 65536;
pub const FMOD_ERR_TOOMANYSAMPLES: FMOD_RESULT = 81;
pub const FMOD_ERR_RECORD_DISCONNECTED: FMOD_RESULT = 80;
pub const FMOD_ERR_NOT_LOCKED: FMOD_RESULT = 79;
pub const FMOD_ERR_ALREADY_LOCKED: FMOD_RESULT = 78;
pub const FMOD_ERR_INVALID_STRING: FMOD_RESULT = 77;
pub const FMOD_ERR_STUDIO_NOT_LOADED: FMOD_RESULT = 76;
pub const FMOD_ERR_STUDIO_UNINITIALIZED: FMOD_RESULT = 75;
pub const FMOD_ERR_EVENT_NOTFOUND: FMOD_RESULT = 74;
pub const FMOD_ERR_EVENT_LIVEUPDATE_TIMEOUT: FMOD_RESULT = 73;
pub const FMOD_ERR_EVENT_LIVEUPDATE_MISMATCH: FMOD_RESULT = 72;
pub const FMOD_ERR_EVENT_LIVEUPDATE_BUSY: FMOD_RESULT = 71;
pub const FMOD_ERR_EVENT_ALREADY_LOADED: FMOD_RESULT = 70;
pub const FMOD_ERR_VERSION: FMOD_RESULT = 69;
pub const FMOD_ERR_UNSUPPORTED: FMOD_RESULT = 68;
pub const FMOD_ERR_UNINITIALIZED: FMOD_RESULT = 67;
pub const FMOD_ERR_UNIMPLEMENTED: FMOD_RESULT = 66;
pub const FMOD_ERR_TRUNCATED: FMOD_RESULT = 65;
pub const FMOD_ERR_TOOMANYCHANNELS: FMOD_RESULT = 64;
pub const FMOD_ERR_TAGNOTFOUND: FMOD_RESULT = 63;
pub const FMOD_ERR_SUBSOUND_CANTMOVE: FMOD_RESULT = 62;
pub const FMOD_ERR_SUBSOUND_ALLOCATED: FMOD_RESULT = 61;
pub const FMOD_ERR_SUBSOUNDS: FMOD_RESULT = 60;
pub const FMOD_ERR_REVERB_INSTANCE: FMOD_RESULT = 59;
pub const FMOD_ERR_REVERB_CHANNELGROUP: FMOD_RESULT = 58;
pub const FMOD_ERR_RECORD: FMOD_RESULT = 57;
pub const FMOD_ERR_PLUGIN_VERSION: FMOD_RESULT = 56;
pub const FMOD_ERR_PLUGIN_RESOURCE: FMOD_RESULT = 55;
pub const FMOD_ERR_PLUGIN_MISSING: FMOD_RESULT = 54;
pub const FMOD_ERR_PLUGIN: FMOD_RESULT = 53;
pub const FMOD_ERR_OUTPUT_NODRIVERS: FMOD_RESULT = 52;
pub const FMOD_ERR_OUTPUT_INIT: FMOD_RESULT = 51;
pub const FMOD_ERR_OUTPUT_FORMAT: FMOD_RESULT = 50;
pub const FMOD_ERR_OUTPUT_DRIVERCALL: FMOD_RESULT = 49;
pub const FMOD_ERR_OUTPUT_CREATEBUFFER: FMOD_RESULT = 48;
pub const FMOD_ERR_OUTPUT_ALLOCATED: FMOD_RESULT = 47;
pub const FMOD_ERR_NOTREADY: FMOD_RESULT = 46;
pub const FMOD_ERR_NET_WOULD_BLOCK: FMOD_RESULT = 45;
pub const FMOD_ERR_NET_URL: FMOD_RESULT = 44;
pub const FMOD_ERR_NET_SOCKET_ERROR: FMOD_RESULT = 43;
pub const FMOD_ERR_NET_CONNECT: FMOD_RESULT = 42;
pub const FMOD_ERR_NEEDSHARDWARE: FMOD_RESULT = 41;
pub const FMOD_ERR_NEEDS3D: FMOD_RESULT = 40;
pub const FMOD_ERR_MEMORY_CANTPOINT: FMOD_RESULT = 39;
pub const FMOD_ERR_MEMORY: FMOD_RESULT = 38;
pub const FMOD_ERR_MAXAUDIBLE: FMOD_RESULT = 37;
pub const FMOD_ERR_INVALID_VECTOR: FMOD_RESULT = 36;
pub const FMOD_ERR_INVALID_THREAD: FMOD_RESULT = 35;
pub const FMOD_ERR_INVALID_SYNCPOINT: FMOD_RESULT = 34;
pub const FMOD_ERR_INVALID_SPEAKER: FMOD_RESULT = 33;
pub const FMOD_ERR_INVALID_POSITION: FMOD_RESULT = 32;
pub const FMOD_ERR_INVALID_PARAM: FMOD_RESULT = 31;
pub const FMOD_ERR_INVALID_HANDLE: FMOD_RESULT = 30;
pub const FMOD_ERR_INVALID_FLOAT: FMOD_RESULT = 29;
pub const FMOD_ERR_INTERNAL: FMOD_RESULT = 28;
pub const FMOD_ERR_INITIALIZED: FMOD_RESULT = 27;
pub const FMOD_ERR_INITIALIZATION: FMOD_RESULT = 26;
pub const FMOD_ERR_HTTP_TIMEOUT: FMOD_RESULT = 25;
pub const FMOD_ERR_HTTP_SERVER_ERROR: FMOD_RESULT = 24;
pub const FMOD_ERR_HTTP_PROXY_AUTH: FMOD_RESULT = 23;
pub const FMOD_ERR_HTTP_ACCESS: FMOD_RESULT = 22;
pub const FMOD_ERR_HTTP: FMOD_RESULT = 21;
pub const FMOD_ERR_HEADER_MISMATCH: FMOD_RESULT = 20;
pub const FMOD_ERR_FORMAT: FMOD_RESULT = 19;
pub const FMOD_ERR_FILE_NOTFOUND: FMOD_RESULT = 18;
pub const FMOD_ERR_FILE_ENDOFDATA: FMOD_RESULT = 17;
pub const FMOD_ERR_FILE_EOF: FMOD_RESULT = 16;
pub const FMOD_ERR_FILE_DISKEJECTED: FMOD_RESULT = 15;
pub const FMOD_ERR_FILE_COULDNOTSEEK: FMOD_RESULT = 14;
pub const FMOD_ERR_FILE_BAD: FMOD_RESULT = 13;
pub const FMOD_ERR_DSP_TYPE: FMOD_RESULT = 12;
pub const FMOD_ERR_DSP_SILENCE: FMOD_RESULT = 11;
pub const FMOD_ERR_DSP_RESERVED: FMOD_RESULT = 10;
pub const FMOD_ERR_DSP_NOTFOUND: FMOD_RESULT = 9;
pub const FMOD_ERR_DSP_INUSE: FMOD_RESULT = 8;
pub const FMOD_ERR_DSP_FORMAT: FMOD_RESULT = 7;
pub const FMOD_ERR_DSP_DONTPROCESS: FMOD_RESULT = 6;
pub const FMOD_ERR_DSP_CONNECTION: FMOD_RESULT = 5;
pub const FMOD_ERR_DMA: FMOD_RESULT = 4;
pub const FMOD_ERR_CHANNEL_STOLEN: FMOD_RESULT = 3;
pub const FMOD_ERR_CHANNEL_ALLOC: FMOD_RESULT = 2;
pub const FMOD_ERR_BADCOMMAND: FMOD_RESULT = 1;
pub const FMOD_OK: FMOD_RESULT = 0;
pub type FMOD_INITFLAGS = libc::c_uint;
pub type FMOD_DEBUG_CALLBACK = Option::<
    unsafe extern "C" fn(
        FMOD_DEBUG_FLAGS,
        *const libc::c_char,
        libc::c_int,
        *const libc::c_char,
        *const libc::c_char,
    ) -> FMOD_RESULT,
>;
pub type FMOD_DEBUG_FLAGS = libc::c_uint;
pub type FMOD_DEBUG_MODE = libc::c_uint;
pub const FMOD_DEBUG_MODE_FORCEINT: FMOD_DEBUG_MODE = 65536;
pub const FMOD_DEBUG_MODE_CALLBACK: FMOD_DEBUG_MODE = 2;
pub const FMOD_DEBUG_MODE_FILE: FMOD_DEBUG_MODE = 1;
pub const FMOD_DEBUG_MODE_TTY: FMOD_DEBUG_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FMOD_VECTOR {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
}

#[inline]
unsafe extern "C" fn FMODError_ToString(mut self_1: FMOD_RESULT) -> cstr {
    match self_1 as libc::c_uint {
        0 => return b"FMOD_OK\0" as *const u8 as *const libc::c_char,
        1 => return b"FMOD_ERR_BADCOMMAND\0" as *const u8 as *const libc::c_char,
        2 => return b"FMOD_ERR_CHANNEL_ALLOC\0" as *const u8 as *const libc::c_char,
        3 => return b"FMOD_ERR_CHANNEL_STOLEN\0" as *const u8 as *const libc::c_char,
        4 => return b"FMOD_ERR_DMA\0" as *const u8 as *const libc::c_char,
        5 => return b"FMOD_ERR_DSP_CONNECTION\0" as *const u8 as *const libc::c_char,
        6 => return b"FMOD_ERR_DSP_DONTPROCESS\0" as *const u8 as *const libc::c_char,
        7 => return b"FMOD_ERR_DSP_FORMAT\0" as *const u8 as *const libc::c_char,
        8 => return b"FMOD_ERR_DSP_INUSE\0" as *const u8 as *const libc::c_char,
        9 => return b"FMOD_ERR_DSP_NOTFOUND\0" as *const u8 as *const libc::c_char,
        10 => return b"FMOD_ERR_DSP_RESERVED\0" as *const u8 as *const libc::c_char,
        11 => return b"FMOD_ERR_DSP_SILENCE\0" as *const u8 as *const libc::c_char,
        12 => return b"FMOD_ERR_DSP_TYPE\0" as *const u8 as *const libc::c_char,
        13 => return b"FMOD_ERR_FILE_BAD\0" as *const u8 as *const libc::c_char,
        14 => return b"FMOD_ERR_FILE_COULDNOTSEEK\0" as *const u8 as *const libc::c_char,
        15 => return b"FMOD_ERR_FILE_DISKEJECTED\0" as *const u8 as *const libc::c_char,
        16 => return b"FMOD_ERR_FILE_EOF\0" as *const u8 as *const libc::c_char,
        17 => return b"FMOD_ERR_FILE_ENDOFDATA\0" as *const u8 as *const libc::c_char,
        18 => return b"FMOD_ERR_FILE_NOTFOUND\0" as *const u8 as *const libc::c_char,
        19 => return b"FMOD_ERR_FORMAT\0" as *const u8 as *const libc::c_char,
        20 => return b"FMOD_ERR_HEADER_MISMATCH\0" as *const u8 as *const libc::c_char,
        21 => return b"FMOD_ERR_HTTP\0" as *const u8 as *const libc::c_char,
        22 => return b"FMOD_ERR_HTTP_ACCESS\0" as *const u8 as *const libc::c_char,
        23 => return b"FMOD_ERR_HTTP_PROXY_AUTH\0" as *const u8 as *const libc::c_char,
        24 => return b"FMOD_ERR_HTTP_SERVER_ERROR\0" as *const u8 as *const libc::c_char,
        25 => return b"FMOD_ERR_HTTP_TIMEOUT\0" as *const u8 as *const libc::c_char,
        26 => return b"FMOD_ERR_INITIALIZATION\0" as *const u8 as *const libc::c_char,
        27 => return b"FMOD_ERR_INITIALIZED\0" as *const u8 as *const libc::c_char,
        28 => return b"FMOD_ERR_INTERNAL\0" as *const u8 as *const libc::c_char,
        29 => return b"FMOD_ERR_INVALID_FLOAT\0" as *const u8 as *const libc::c_char,
        30 => return b"FMOD_ERR_INVALID_HANDLE\0" as *const u8 as *const libc::c_char,
        31 => return b"FMOD_ERR_INVALID_PARAM\0" as *const u8 as *const libc::c_char,
        32 => return b"FMOD_ERR_INVALID_POSITION\0" as *const u8 as *const libc::c_char,
        33 => return b"FMOD_ERR_INVALID_SPEAKER\0" as *const u8 as *const libc::c_char,
        34 => return b"FMOD_ERR_INVALID_SYNCPOINT\0" as *const u8 as *const libc::c_char,
        35 => return b"FMOD_ERR_INVALID_THREAD\0" as *const u8 as *const libc::c_char,
        36 => return b"FMOD_ERR_INVALID_VECTOR\0" as *const u8 as *const libc::c_char,
        37 => return b"FMOD_ERR_MAXAUDIBLE\0" as *const u8 as *const libc::c_char,
        38 => return b"FMOD_ERR_MEMORY\0" as *const u8 as *const libc::c_char,
        39 => return b"FMOD_ERR_MEMORY_CANTPOINT\0" as *const u8 as *const libc::c_char,
        40 => return b"FMOD_ERR_NEEDS3D\0" as *const u8 as *const libc::c_char,
        41 => return b"FMOD_ERR_NEEDSHARDWARE\0" as *const u8 as *const libc::c_char,
        42 => return b"FMOD_ERR_NET_CONNECT\0" as *const u8 as *const libc::c_char,
        43 => return b"FMOD_ERR_NET_SOCKET_ERROR\0" as *const u8 as *const libc::c_char,
        44 => return b"FMOD_ERR_NET_URL\0" as *const u8 as *const libc::c_char,
        45 => return b"FMOD_ERR_NET_WOULD_BLOCK\0" as *const u8 as *const libc::c_char,
        46 => return b"FMOD_ERR_NOTREADY\0" as *const u8 as *const libc::c_char,
        47 => return b"FMOD_ERR_OUTPUT_ALLOCATED\0" as *const u8 as *const libc::c_char,
        48 => {
            return b"FMOD_ERR_OUTPUT_CREATEBUFFER\0" as *const u8 as *const libc::c_char;
        }
        49 => return b"FMOD_ERR_OUTPUT_DRIVERCALL\0" as *const u8 as *const libc::c_char,
        50 => return b"FMOD_ERR_OUTPUT_FORMAT\0" as *const u8 as *const libc::c_char,
        51 => return b"FMOD_ERR_OUTPUT_INIT\0" as *const u8 as *const libc::c_char,
        52 => return b"FMOD_ERR_OUTPUT_NODRIVERS\0" as *const u8 as *const libc::c_char,
        53 => return b"FMOD_ERR_PLUGIN\0" as *const u8 as *const libc::c_char,
        54 => return b"FMOD_ERR_PLUGIN_MISSING\0" as *const u8 as *const libc::c_char,
        55 => return b"FMOD_ERR_PLUGIN_RESOURCE\0" as *const u8 as *const libc::c_char,
        56 => return b"FMOD_ERR_PLUGIN_VERSION\0" as *const u8 as *const libc::c_char,
        57 => return b"FMOD_ERR_RECORD\0" as *const u8 as *const libc::c_char,
        58 => {
            return b"FMOD_ERR_REVERB_CHANNELGROUP\0" as *const u8 as *const libc::c_char;
        }
        59 => return b"FMOD_ERR_REVERB_INSTANCE\0" as *const u8 as *const libc::c_char,
        60 => return b"FMOD_ERR_SUBSOUNDS\0" as *const u8 as *const libc::c_char,
        61 => return b"FMOD_ERR_SUBSOUND_ALLOCATED\0" as *const u8 as *const libc::c_char,
        62 => return b"FMOD_ERR_SUBSOUND_CANTMOVE\0" as *const u8 as *const libc::c_char,
        63 => return b"FMOD_ERR_TAGNOTFOUND\0" as *const u8 as *const libc::c_char,
        64 => return b"FMOD_ERR_TOOMANYCHANNELS\0" as *const u8 as *const libc::c_char,
        65 => return b"FMOD_ERR_TRUNCATED\0" as *const u8 as *const libc::c_char,
        66 => return b"FMOD_ERR_UNIMPLEMENTED\0" as *const u8 as *const libc::c_char,
        67 => return b"FMOD_ERR_UNINITIALIZED\0" as *const u8 as *const libc::c_char,
        68 => return b"FMOD_ERR_UNSUPPORTED\0" as *const u8 as *const libc::c_char,
        69 => return b"FMOD_ERR_VERSION\0" as *const u8 as *const libc::c_char,
        70 => {
            return b"FMOD_ERR_EVENT_ALREADY_LOADED\0" as *const u8 as *const libc::c_char;
        }
        71 => {
            return b"FMOD_ERR_EVENT_LIVEUPDATE_BUSY\0" as *const u8
                as *const libc::c_char;
        }
        72 => {
            return b"FMOD_ERR_EVENT_LIVEUPDATE_MISMATCH\0" as *const u8
                as *const libc::c_char;
        }
        73 => {
            return b"FMOD_ERR_EVENT_LIVEUPDATE_TIMEOUT\0" as *const u8
                as *const libc::c_char;
        }
        74 => return b"FMOD_ERR_EVENT_NOTFOUND\0" as *const u8 as *const libc::c_char,
        75 => {
            return b"FMOD_ERR_STUDIO_UNINITIALIZED\0" as *const u8 as *const libc::c_char;
        }
        76 => return b"FMOD_ERR_STUDIO_NOT_LOADED\0" as *const u8 as *const libc::c_char,
        77 => return b"FMOD_ERR_INVALID_STRING\0" as *const u8 as *const libc::c_char,
        78 => return b"FMOD_ERR_ALREADY_LOCKED\0" as *const u8 as *const libc::c_char,
        79 => return b"FMOD_ERR_NOT_LOCKED\0" as *const u8 as *const libc::c_char,
        80 => {
            return b"FMOD_ERR_RECORD_DISCONNECTED\0" as *const u8 as *const libc::c_char;
        }
        81 => return b"FMOD_ERR_TOOMANYSAMPLES\0" as *const u8 as *const libc::c_char,
        65536 => return b"FMOD_RESULT_FORCEINT\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    return b"Unknown Error\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn FMOD_ErrorString(mut errcode: FMOD_RESULT) -> *const libc::c_char {
    match errcode as libc::c_uint {
        0 => return b"No errors.\0" as *const u8 as *const libc::c_char,
        1 => {
            return b"Tried to call a function on a data type that does not allow this type of functionality (ie calling Sound::lock on a streaming sound).\0"
                as *const u8 as *const libc::c_char;
        }
        2 => {
            return b"Error trying to allocate a channel.\0" as *const u8
                as *const libc::c_char;
        }
        3 => {
            return b"The specified channel has been reused to play another sound.\0"
                as *const u8 as *const libc::c_char;
        }
        4 => {
            return b"DMA Failure.  See debug output for more information.\0" as *const u8
                as *const libc::c_char;
        }
        5 => {
            return b"DSP connection error.  Connection possibly caused a cyclic dependency or connected dsps with incompatible buffer counts.\0"
                as *const u8 as *const libc::c_char;
        }
        6 => {
            return b"DSP return code from a DSP process query callback.  Tells mixer not to call the process callback and therefore not consume CPU.  Use this to optimize the DSP graph.\0"
                as *const u8 as *const libc::c_char;
        }
        7 => {
            return b"DSP Format error.  A DSP unit may have attempted to connect to this network with the wrong format, or a matrix may have been set with the wrong size if the target unit has a specified channel map.\0"
                as *const u8 as *const libc::c_char;
        }
        8 => {
            return b"DSP is already in the mixer's DSP network. It must be removed before being reinserted or released.\0"
                as *const u8 as *const libc::c_char;
        }
        9 => {
            return b"DSP connection error.  Couldn't find the DSP unit specified.\0"
                as *const u8 as *const libc::c_char;
        }
        10 => {
            return b"DSP operation error.  Cannot perform operation on this DSP as it is reserved by the system.\0"
                as *const u8 as *const libc::c_char;
        }
        11 => {
            return b"DSP return code from a DSP process query callback.  Tells mixer silence would be produced from read, so go idle and not consume CPU.  Use this to optimize the DSP graph.\0"
                as *const u8 as *const libc::c_char;
        }
        12 => {
            return b"DSP operation cannot be performed on a DSP of this type.\0"
                as *const u8 as *const libc::c_char;
        }
        13 => return b"Error loading file.\0" as *const u8 as *const libc::c_char,
        14 => {
            return b"Couldn't perform seek operation.  This is a limitation of the medium (ie netstreams) or the file format.\0"
                as *const u8 as *const libc::c_char;
        }
        15 => {
            return b"Media was ejected while reading.\0" as *const u8
                as *const libc::c_char;
        }
        16 => {
            return b"End of file unexpectedly reached while trying to read essential data (truncated?).\0"
                as *const u8 as *const libc::c_char;
        }
        17 => {
            return b"End of current chunk reached while trying to read data.\0"
                as *const u8 as *const libc::c_char;
        }
        18 => return b"File not found.\0" as *const u8 as *const libc::c_char,
        19 => {
            return b"Unsupported file or audio format.\0" as *const u8
                as *const libc::c_char;
        }
        20 => {
            return b"There is a version mismatch between the FMOD header and either the FMOD Studio library or the FMOD Low Level library.\0"
                as *const u8 as *const libc::c_char;
        }
        21 => {
            return b"A HTTP error occurred. This is a catch-all for HTTP errors not listed elsewhere.\0"
                as *const u8 as *const libc::c_char;
        }
        22 => {
            return b"The specified resource requires authentication or is forbidden.\0"
                as *const u8 as *const libc::c_char;
        }
        23 => {
            return b"Proxy authentication is required to access the specified resource.\0"
                as *const u8 as *const libc::c_char;
        }
        24 => {
            return b"A HTTP server error occurred.\0" as *const u8 as *const libc::c_char;
        }
        25 => return b"The HTTP request timed out.\0" as *const u8 as *const libc::c_char,
        26 => {
            return b"FMOD was not initialized correctly to support this function.\0"
                as *const u8 as *const libc::c_char;
        }
        27 => {
            return b"Cannot call this command after System::init.\0" as *const u8
                as *const libc::c_char;
        }
        28 => {
            return b"An error occurred that wasn't supposed to.  Contact support.\0"
                as *const u8 as *const libc::c_char;
        }
        29 => {
            return b"Value passed in was a NaN, Inf or denormalized float.\0"
                as *const u8 as *const libc::c_char;
        }
        30 => {
            return b"An invalid object handle was used.\0" as *const u8
                as *const libc::c_char;
        }
        31 => {
            return b"An invalid parameter was passed to this function.\0" as *const u8
                as *const libc::c_char;
        }
        32 => {
            return b"An invalid seek position was passed to this function.\0"
                as *const u8 as *const libc::c_char;
        }
        33 => {
            return b"An invalid speaker was passed to this function based on the current speaker mode.\0"
                as *const u8 as *const libc::c_char;
        }
        34 => {
            return b"The syncpoint did not come from this sound handle.\0" as *const u8
                as *const libc::c_char;
        }
        35 => {
            return b"Tried to call a function on a thread that is not supported.\0"
                as *const u8 as *const libc::c_char;
        }
        36 => {
            return b"The vectors passed in are not unit length, or perpendicular.\0"
                as *const u8 as *const libc::c_char;
        }
        37 => {
            return b"Reached maximum audible playback count for this sound's soundgroup.\0"
                as *const u8 as *const libc::c_char;
        }
        38 => {
            return b"Not enough memory or resources.\0" as *const u8
                as *const libc::c_char;
        }
        39 => {
            return b"Can't use FMOD_OPENMEMORY_POINT on non PCM source data, or non mp3/xma/adpcm data if FMOD_CREATECOMPRESSEDSAMPLE was used.\0"
                as *const u8 as *const libc::c_char;
        }
        40 => {
            return b"Tried to call a command on a 2d sound when the command was meant for 3d sound.\0"
                as *const u8 as *const libc::c_char;
        }
        41 => {
            return b"Tried to use a feature that requires hardware support.\0"
                as *const u8 as *const libc::c_char;
        }
        42 => {
            return b"Couldn't connect to the specified host.\0" as *const u8
                as *const libc::c_char;
        }
        43 => {
            return b"A socket error occurred.  This is a catch-all for socket-related errors not listed elsewhere.\0"
                as *const u8 as *const libc::c_char;
        }
        44 => {
            return b"The specified URL couldn't be resolved.\0" as *const u8
                as *const libc::c_char;
        }
        45 => {
            return b"Operation on a non-blocking socket could not complete immediately.\0"
                as *const u8 as *const libc::c_char;
        }
        46 => {
            return b"Operation could not be performed because specified sound/DSP connection is not ready.\0"
                as *const u8 as *const libc::c_char;
        }
        47 => {
            return b"Error initializing output device, but more specifically, the output device is already in use and cannot be reused.\0"
                as *const u8 as *const libc::c_char;
        }
        48 => {
            return b"Error creating hardware sound buffer.\0" as *const u8
                as *const libc::c_char;
        }
        49 => {
            return b"A call to a standard soundcard driver failed, which could possibly mean a bug in the driver or resources were missing or exhausted.\0"
                as *const u8 as *const libc::c_char;
        }
        50 => {
            return b"Soundcard does not support the specified format.\0" as *const u8
                as *const libc::c_char;
        }
        51 => {
            return b"Error initializing output device.\0" as *const u8
                as *const libc::c_char;
        }
        52 => {
            return b"The output device has no drivers installed.  If pre-init, FMOD_OUTPUT_NOSOUND is selected as the output mode.  If post-init, the function just fails.\0"
                as *const u8 as *const libc::c_char;
        }
        53 => {
            return b"An unspecified error has been returned from a plugin.\0"
                as *const u8 as *const libc::c_char;
        }
        54 => {
            return b"A requested output, dsp unit type or codec was not available.\0"
                as *const u8 as *const libc::c_char;
        }
        55 => {
            return b"A resource that the plugin requires cannot be allocated or found. (ie the DLS file for MIDI playback)\0"
                as *const u8 as *const libc::c_char;
        }
        56 => {
            return b"A plugin was built with an unsupported SDK version.\0" as *const u8
                as *const libc::c_char;
        }
        57 => {
            return b"An error occurred trying to initialize the recording device.\0"
                as *const u8 as *const libc::c_char;
        }
        58 => {
            return b"Reverb properties cannot be set on this channel because a parent channelgroup owns the reverb connection.\0"
                as *const u8 as *const libc::c_char;
        }
        59 => {
            return b"Specified instance in FMOD_REVERB_PROPERTIES couldn't be set. Most likely because it is an invalid instance number or the reverb doesn't exist.\0"
                as *const u8 as *const libc::c_char;
        }
        60 => {
            return b"The error occurred because the sound referenced contains subsounds when it shouldn't have, or it doesn't contain subsounds when it should have.  The operation may also not be able to be performed on a parent sound.\0"
                as *const u8 as *const libc::c_char;
        }
        61 => {
            return b"This subsound is already being used by another sound, you cannot have more than one parent to a sound.  Null out the other parent's entry first.\0"
                as *const u8 as *const libc::c_char;
        }
        62 => {
            return b"Shared subsounds cannot be replaced or moved from their parent stream, such as when the parent stream is an FSB file.\0"
                as *const u8 as *const libc::c_char;
        }
        63 => {
            return b"The specified tag could not be found or there are no tags.\0"
                as *const u8 as *const libc::c_char;
        }
        64 => {
            return b"The sound created exceeds the allowable input channel count.  This can be increased using the 'maxinputchannels' parameter in System::setSoftwareFormat.\0"
                as *const u8 as *const libc::c_char;
        }
        65 => {
            return b"The retrieved string is too long to fit in the supplied buffer and has been truncated.\0"
                as *const u8 as *const libc::c_char;
        }
        66 => {
            return b"Something in FMOD hasn't been implemented when it should be! contact support!\0"
                as *const u8 as *const libc::c_char;
        }
        67 => {
            return b"This command failed because System::init or System::setDriver was not called.\0"
                as *const u8 as *const libc::c_char;
        }
        68 => {
            return b"A command issued was not supported by this object.  Possibly a plugin without certain callbacks specified.\0"
                as *const u8 as *const libc::c_char;
        }
        69 => {
            return b"The version number of this file format is not supported.\0"
                as *const u8 as *const libc::c_char;
        }
        70 => {
            return b"The specified bank has already been loaded.\0" as *const u8
                as *const libc::c_char;
        }
        71 => {
            return b"The live update connection failed due to the game already being connected.\0"
                as *const u8 as *const libc::c_char;
        }
        72 => {
            return b"The live update connection failed due to the game data being out of sync with the tool.\0"
                as *const u8 as *const libc::c_char;
        }
        73 => {
            return b"The live update connection timed out.\0" as *const u8
                as *const libc::c_char;
        }
        74 => {
            return b"The requested event, parameter, bus or vca could not be found.\0"
                as *const u8 as *const libc::c_char;
        }
        75 => {
            return b"The Studio::System object is not yet initialized.\0" as *const u8
                as *const libc::c_char;
        }
        76 => {
            return b"The specified resource is not loaded, so it can't be unloaded.\0"
                as *const u8 as *const libc::c_char;
        }
        77 => {
            return b"An invalid string was passed to this function.\0" as *const u8
                as *const libc::c_char;
        }
        78 => {
            return b"The specified resource is already locked.\0" as *const u8
                as *const libc::c_char;
        }
        79 => {
            return b"The specified resource is not locked, so it can't be unlocked.\0"
                as *const u8 as *const libc::c_char;
        }
        80 => {
            return b"The specified recording driver has been disconnected.\0"
                as *const u8 as *const libc::c_char;
        }
        81 => {
            return b"The length provided exceeds the allowable limit.\0" as *const u8
                as *const libc::c_char;
        }
        _ => return b"Unknown error.\0" as *const u8 as *const libc::c_char,
    };
}
#[inline]
unsafe extern "C" fn FMOD_CheckError(
    mut result: FMOD_RESULT,
    mut file: cstr,
    mut line: libc::c_int,
    mut func: cstr,
) {
    if result as libc::c_uint != FMOD_OK as libc::c_int as libc::c_uint {
        Fatal(
            b"%s: %s\n%s\n  [%s @ Line %d]\0" as *const u8 as *const libc::c_char,
            func,
            FMODError_ToString(result),
            FMOD_ErrorString(result),
            file,
            line,
        );
    }
}
static mut self_0: Audio = Audio {
    handle: 0 as *const FMOD_SYSTEM as *mut FMOD_SYSTEM,
    descMap: 0 as *const StrMap as *mut StrMap,
    soundPool: 0 as *const MemPool as *mut MemPool,
    playingSounds_size: 0,
    playingSounds_capacity: 0,
    playingSounds_data: 0 as *const *mut Sound as *mut *mut Sound,
    freeingSounds_size: 0,
    freeingSounds_capacity: 0,
    freeingSounds_data: 0 as *const *mut Sound as *mut *mut Sound,
    autoPos: 0 as *const Vec3f,
    autoVel: 0 as *const Vec3f,
    autoFwd: 0 as *const Vec3f,
    autoUp: 0 as *const Vec3f,
};
#[no_mangle]
pub unsafe extern "C" fn Audio_Init() {
    let mut flags: FMOD_DEBUG_FLAGS = 0 as libc::c_int as FMOD_DEBUG_FLAGS;
    flags |= 0 as libc::c_int as libc::c_uint;
    let mut res: FMOD_RESULT = FMOD_OK;
    res = FMOD_Debug_Initialize(
        flags,
        FMOD_DEBUG_MODE_FILE,
        None,
        b"log/fmod.txt\0" as *const u8 as *const libc::c_char,
    );
    if res as libc::c_uint != FMOD_OK as libc::c_int as libc::c_uint
        && res as libc::c_uint != FMOD_ERR_UNSUPPORTED as libc::c_int as libc::c_uint
    {
        FMOD_CheckError(
            res,
            b"/Users/dgavedissian/Work/ltheory/libphx/src/Audio.c\0" as *const u8
                as *const libc::c_char,
            39 as libc::c_int,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"Audio_Init\0"))
                .as_ptr(),
        );
    }
    FMOD_CheckError(
        FMOD_System_Create(&mut self_0.handle, 0x20208 as libc::c_int as libc::c_uint),
        b"/Users/dgavedissian/Work/ltheory/libphx/src/Audio.c\0" as *const u8
            as *const libc::c_char,
        43 as libc::c_int,
        (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"Audio_Init\0"))
            .as_ptr(),
    );
    let mut version: uint = 0;
    FMOD_CheckError(
        FMOD_System_GetVersion(self_0.handle, &mut version),
        b"/Users/dgavedissian/Work/ltheory/libphx/src/Audio.c\0" as *const u8
            as *const libc::c_char,
        46 as libc::c_int,
        (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"Audio_Init\0"))
            .as_ptr(),
    );
    if version < 0x20208 as libc::c_int as libc::c_uint {
        Fatal(
            b"Audio_Create: FMOD library link/compile version mismatch\0" as *const u8
                as *const libc::c_char,
        );
    }
    let mut flags_0: FMOD_INITFLAGS = 0 as libc::c_int as FMOD_INITFLAGS;
    flags_0 |= 0 as libc::c_int as libc::c_uint;
    flags_0 |= 0x4 as libc::c_int as libc::c_uint;
    flags_0 |= 0x200 as libc::c_int as libc::c_uint;
    FMOD_CheckError(
        FMOD_System_Init(
            self_0.handle,
            1024 as libc::c_int,
            flags_0,
            0 as *mut libc::c_void,
        ),
        b"/Users/dgavedissian/Work/ltheory/libphx/src/Audio.c\0" as *const u8
            as *const libc::c_char,
        59 as libc::c_int,
        (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"Audio_Init\0"))
            .as_ptr(),
    );
    self_0.descMap = StrMap_Create(128 as libc::c_int as uint32);
    self_0
        .soundPool = MemPool_Create(
        ::core::mem::size_of::<Sound>() as usize as uint32,
        128 as libc::c_int as uint32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Audio_Free() {
    FMOD_CheckError(
        FMOD_System_Release(self_0.handle),
        b"/Users/dgavedissian/Work/ltheory/libphx/src/Audio.c\0" as *const u8
            as *const libc::c_char,
        69 as libc::c_int,
        (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"Audio_Free\0"))
            .as_ptr(),
    );
    StrMap_Free(self_0.descMap);
    MemPool_Free(self_0.soundPool);
    MemFree(self_0.playingSounds_data as *const libc::c_void);
    MemFree(self_0.freeingSounds_data as *const libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_AttachListenerPos(
    mut pos: *const Vec3f,
    mut vel: *const Vec3f,
    mut fwd: *const Vec3f,
    mut up: *const Vec3f,
) {
    self_0.autoPos = pos;
    self_0.autoVel = vel;
    self_0.autoFwd = fwd;
    self_0.autoUp = up;
    Audio_SetListenerPos(pos, vel, fwd, up);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_Set3DSettings(
    mut doppler: libc::c_float,
    mut scale: libc::c_float,
    mut rolloff: libc::c_float,
) {
    FMOD_CheckError(
        FMOD_System_Set3DSettings(self_0.handle, doppler, scale, rolloff),
        b"/Users/dgavedissian/Work/ltheory/libphx/src/Audio.c\0" as *const u8
            as *const libc::c_char,
        85 as libc::c_int,
        (*::core::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"Audio_Set3DSettings\0"))
            .as_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn Audio_SetListenerPos(
    mut pos: *const Vec3f,
    mut vel: *const Vec3f,
    mut fwd: *const Vec3f,
    mut up: *const Vec3f,
) {
    FMOD_CheckError(
        FMOD_System_Set3DListenerAttributes(
            self_0.handle,
            0 as libc::c_int,
            pos as *mut FMOD_VECTOR,
            vel as *mut FMOD_VECTOR,
            fwd as *mut FMOD_VECTOR,
            up as *mut FMOD_VECTOR,
        ),
        b"/Users/dgavedissian/Work/ltheory/libphx/src/Audio.c\0" as *const u8
            as *const libc::c_char,
        106 as libc::c_int,
        (*::core::mem::transmute::<
            &[u8; 21],
            &[libc::c_char; 21],
        >(b"Audio_SetListenerPos\0"))
            .as_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn Audio_Update() {
    FMOD_CheckError(
        FMOD_System_Update(self_0.handle),
        b"/Users/dgavedissian/Work/ltheory/libphx/src/Audio.c\0" as *const u8
            as *const libc::c_char,
        110 as libc::c_int,
        (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"Audio_Update\0"))
            .as_ptr(),
    );
    Audio_SetListenerPos(self_0.autoPos, self_0.autoVel, self_0.autoFwd, self_0.autoUp);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < self_0.playingSounds_size {
        let mut sound: *mut Sound = *(self_0.playingSounds_data).offset(i as isize);
        if !Sound_IsFreed(sound) && Sound_IsPlaying(sound) as libc::c_int != 0 {
            Sound_Update(sound);
        } else {
            self_0.playingSounds_size -= 1;
            let fresh0 = i;
            i = i - 1;
            let ref mut fresh1 = *(self_0.playingSounds_data).offset(fresh0 as isize);
            *fresh1 = *(self_0.playingSounds_data)
                .offset(self_0.playingSounds_size as isize);
        }
        i += 1;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < self_0.freeingSounds_size {
        let mut sound_0: *mut Sound = *(self_0.freeingSounds_data).offset(i_0 as isize);
        Audio_DeallocSound(sound_0);
        i_0 += 1;
    }
    self_0.freeingSounds_size = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_GetLoadedCount() -> int32 {
    let mut size: uint32 = StrMap_GetSize(self_0.descMap);
    return size as int32;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_GetPlayingCount() -> int32 {
    return self_0.playingSounds_size;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_GetTotalCount() -> int32 {
    let mut size: uint32 = MemPool_GetSize(self_0.soundPool);
    return size as int32;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_GetHandle() -> *mut libc::c_void {
    return self_0.handle as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_AllocSoundDesc(mut name: cstr) -> *mut SoundDesc {
    let mut desc: *mut SoundDesc = StrMap_Get(self_0.descMap, name) as *mut SoundDesc;
    if desc.is_null() {
        desc = MemAllocZero(::core::mem::size_of::<SoundDesc>())
            as *mut SoundDesc;
        StrMap_Set(self_0.descMap, name, desc as *mut libc::c_void);
    }
    return desc;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_DeallocSoundDesc(mut desc: *mut SoundDesc) {
    StrMap_Remove(self_0.descMap, (*desc).name);
    MemFree(desc as *const libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_AllocSound() -> *mut Sound {
    return MemPool_Alloc(self_0.soundPool) as *mut Sound;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_DeallocSound(mut sound: *mut Sound) {
    MemPool_Dealloc(self_0.soundPool, sound as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_SoundStateChanged(mut sound: *mut Sound) {
    if Sound_IsFreed(sound) {
        if (self_0.freeingSounds_capacity == self_0.freeingSounds_size) as libc::c_int
            as libc::c_long != 0
        {
            self_0
                .freeingSounds_capacity = if self_0.freeingSounds_capacity != 0 {
                self_0.freeingSounds_capacity * 2 as libc::c_int
            } else {
                1 as libc::c_int
            };
            let mut elemSize: usize = ::core::mem::size_of::<*mut Sound>();
            let mut pData: *mut *mut libc::c_void = &mut self_0.freeingSounds_data
                as *mut *mut *mut Sound as *mut *mut libc::c_void;
            *pData = MemRealloc(
                self_0.freeingSounds_data as *mut libc::c_void,
                (self_0.freeingSounds_capacity as usize).wrapping_mul(elemSize as usize),
            );
        }
        let fresh2 = self_0.freeingSounds_size;
        self_0.freeingSounds_size = self_0.freeingSounds_size + 1;
        let ref mut fresh3 = *(self_0.freeingSounds_data).offset(fresh2 as isize);
        *fresh3 = sound;
    } else if Sound_IsPlaying(sound) {
        if (self_0.playingSounds_capacity == self_0.playingSounds_size) as libc::c_int
            as libc::c_long != 0
        {
            self_0
                .playingSounds_capacity = if self_0.playingSounds_capacity != 0 {
                self_0.playingSounds_capacity * 2 as libc::c_int
            } else {
                1 as libc::c_int
            };
            let mut elemSize_0: usize = ::core::mem::size_of::<*mut Sound>();
            let mut pData_0: *mut *mut libc::c_void = &mut self_0.playingSounds_data
                as *mut *mut *mut Sound as *mut *mut libc::c_void;
            *pData_0 = MemRealloc(
                self_0.playingSounds_data as *mut libc::c_void,
                (self_0.playingSounds_capacity as usize).wrapping_mul(elemSize_0 as usize),
            );
        }
        let fresh4 = self_0.playingSounds_size;
        self_0.playingSounds_size = self_0.playingSounds_size + 1;
        let ref mut fresh5 = *(self_0.playingSounds_data).offset(fresh4 as isize);
        *fresh5 = sound;
    }
}
