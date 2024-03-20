#![allow(non_snake_case)]
pub type AudioFormatFlags = u32;
pub type AudioFormatID = u32;

#[repr(C)]
pub struct AudioStreamBasicDescription {
    pub mFormatID: AudioFormatID,
    pub mFormatFlags: AudioFormatFlags,
    pub mSampleRate: f64,
    pub mBitsPerChannel: u32,
    pub mBytesPerFrame: u32,
    pub mChannelsPerFrame: u32,
    pub mBytesPerPacket: u32,
    pub mFramesPerPacket: u32,
    pub mReserved: u32,
}
