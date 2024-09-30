use std::{os::raw::c_void, ptr::NonNull};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioBuffer {
    pub number_channels: u32,
    pub data_byte_size: u32,
    pub data: NonNull<c_void>,
}

impl AudioBuffer {
    pub fn new(number_channels: u32, data_byte_size: u32, data: NonNull<c_void>) -> Self {
        Self {
            number_channels,
            data_byte_size,
            data,
        }
    }
}
