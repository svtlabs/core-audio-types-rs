use std::{ffi::c_void, ptr::NonNull};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioValueTranslation {
    pub input_data: NonNull<c_void>,
    pub output_data: NonNull<c_void>,
    pub input_data_size: u32,
    pub output_data_size: u32,
}

impl AudioValueTranslation {
    pub fn new(
        input_data: NonNull<c_void>,
        output_data: NonNull<c_void>,
        input_data_size: u32,
        output_data_size: u32,
    ) -> Self {
        Self {
            input_data,
            output_data,
            input_data_size,
            output_data_size,
        }
    }
}
