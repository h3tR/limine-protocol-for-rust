use crate::requests::LimineRequest;
use core::mem::MaybeUninit;
use crate::{impl_liminine_req, LimineReqId};
use core::ffi::{c_char, CStr};

#[repr(C, align(8))]
pub struct BootloaderInfoRequest{
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>
}

impl BootloaderInfoRequest {
    pub const fn new(revision: u64) -> Self {
        Self {
            id: LimineReqId::new([0xf55038d8e2a1202f, 0x279426fcf5f59740]),
            revision,
            resp: MaybeUninit::uninit()
        }
    }
}

impl_liminine_req!(BootloaderInfoRequest, BootloaderInfoResponse);

#[repr(C, align(8))]
pub struct BootloaderInfoResponse {
    revision: u64,
    name: *const u8,
    version: *const u8
}
impl BootloaderInfoResponse {
    pub fn get_name(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.name as *const c_char).to_str().unwrap()
        }
    }

    pub fn get_version(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.version as *const c_char).to_str().unwrap()
        }
    }
}