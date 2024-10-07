use crate::audio_buffer::AudioBuffer;
const MAX_AUDIO_BUFFERS: usize = 8;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioBufferList {
    pub number_buffers: u32,
    pub buffers: [AudioBuffer; MAX_AUDIO_BUFFERS],
}


