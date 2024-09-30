use crate::audio_buffer::AudioBuffer;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioBufferList<'a> {
    pub number_buffers: u32,
    pub buffers: &'a [AudioBuffer],
}

impl<'a> AudioBufferList<'a> {
    pub fn new(number_buffers: u32, buffers: &'a [AudioBuffer]) -> Self {
        Self {
            number_buffers,
            buffers,
        }
    }
}
