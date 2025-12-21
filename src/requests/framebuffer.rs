use crate::requests::LimineRequest;
use core::mem::MaybeUninit;
use core::slice::{from_raw_parts};
use crate::{impl_liminine_req, LimineReqId};
use crate::util::PointerSlice;

#[repr(C, align(8))]
pub struct FramebufferRequest {
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>
}

impl FramebufferRequest {
    pub const fn new(revision: u64) -> Self {
        Self {
            id: LimineReqId::new([0x9d5827dcd881dd75, 0xa3148604f6fab11b]),
            revision,
            resp: MaybeUninit::uninit()
        }
    }
}

impl_liminine_req!(FramebufferRequest, FramebufferResponse);


#[repr(C, align(8))]
pub struct FramebufferResponse {
    revision: u64,
    framebuffer_count: u64,
    framebuffers: *const *const Framebuffer
}

impl FramebufferResponse {
    pub fn get_framebuffers(&self) -> PointerSlice<Framebuffer> {
        PointerSlice::from(unsafe {
            from_raw_parts(self.framebuffers, self.framebuffer_count as usize)
        })
    }
}


#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct Framebuffer {
    pub address: usize,
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
    pub bpp: u16,
    pub memory_model: u8,
    pub red_mask_size: u8,
    pub red_mask_shift: u8,
    pub green_mask_size: u8,
    pub green_mask_shift: u8,
    pub blue_mask_size: u8,
    pub blue_mask_shift: u8,
    _unused: [u8; 7],
    pub edid_size: u64,
    pub edid_address: usize,

    //Response revision 1
    pub video_mode_count: u64,
    video_modes: *const *const VideoMode
}

impl Framebuffer {
    pub fn get_video_mode(&self) -> PointerSlice<VideoMode> {
        PointerSlice::from(unsafe {
            from_raw_parts(self.video_modes, self.video_mode_count as usize)
        })
    }
}

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct VideoMode {
    pitch: u64,
    width: u64,
    height: u64,
    bpp: u16,
    memory_model: u8,
    red_mask_size: u8,
    red_mask_shift: u8,
    green_mask_size: u8,
    green_mask_shift: u8,
    blue_mask_size: u8,
    blue_mask_shift: u8,
}