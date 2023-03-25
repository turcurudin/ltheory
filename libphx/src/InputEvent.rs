use crate::internal::Memory::*;
use crate::Button::*;
use crate::Button::*;
use crate::Common::*;
use crate::Device::*;
use crate::DeviceType::*;
use crate::Math::Vec3;
use crate::State::*;
use crate::State::*;
use libc;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct InputEvent {
    pub timestamp: u32,
    pub device: Device,
    pub button: Button,
    pub value: f32,
    pub state: State,
}

#[no_mangle]
pub unsafe extern "C" fn InputEvent_ToString(ie: *mut InputEvent) -> *const libc::c_char {
    static mut buffer: [libc::c_char; 512] = [0; 512];
    libc::snprintf(
        buffer.as_mut_ptr(),
        (std::mem::size_of::<[libc::c_char; 512]>())
            .wrapping_div(std::mem::size_of::<libc::c_char>())
           ,
        c_str!("Event %p\n\tTimestamp: %i\n\tDevice:    %s\n\tButton:    %s\n\tValue:     %.2f\n\tState:     %s"),
        ie,
        (*ie).timestamp,
        Device_ToString(&mut (*ie).device),
        Button_ToString((*ie).button),
        (*ie).value as f64,
        State_ToString((*ie).state),
    );
    buffer.as_mut_ptr() as *const libc::c_char
}
