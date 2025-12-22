//![Higher Half Direct Map Feature](https://codeberg.org/Limine/limine-protocol/src/branch/trunk/PROTOCOL.md#hhdm-higher-half-direct-map-feature)

use crate::requests::LimineRequest;
use core::mem::MaybeUninit;
use crate::{impl_liminine_req, LimineReqId};

#[repr(C, align(8))]
pub struct HigherHalfDirectMapRequest{
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>
}

impl HigherHalfDirectMapRequest {
    pub const fn new(revision: u64) -> Self {
        Self {
            id: LimineReqId::new([0x48dcf1cb8ad2b852, 0x63984e959a98244b]),
            revision,
            resp: MaybeUninit::uninit()
        }
    }
}

impl_liminine_req!(HigherHalfDirectMapRequest, HigherHalfDirectMapResponse);

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct HigherHalfDirectMapResponse {
    revision: u64,
    pub offset: u64
}