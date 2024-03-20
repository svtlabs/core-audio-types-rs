#![allow(non_snake_case)]
#[repr(C)]
pub struct AudioStreamPacketDescription {
    pub mStartOffset: u64,
    pub mVariableFramesInPacket: u32,
    pub mDataByteSize: u32,
}

impl AudioStreamPacketDescription {
    pub fn new(mStartOffset: u64, mVariableFramesInPacket: u32, mDataByteSize: u32) -> Self {
        Self {
            mStartOffset,
            mVariableFramesInPacket,
            mDataByteSize,
        }
    }
}
