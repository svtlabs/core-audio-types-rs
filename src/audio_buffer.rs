use std::ptr::NonNull;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioBuffer<T = u8> {
    pub number_channels: u32,
    pub data_byte_size: u32,
    pub data: NonNull<T>,
}

impl<T> AudioBuffer<T> {
    pub fn new(number_channels: u32, data_byte_size: u32, data: NonNull<T>) -> Self {
        Self {
            number_channels,
            data_byte_size,
            data,
        }
    }
}
