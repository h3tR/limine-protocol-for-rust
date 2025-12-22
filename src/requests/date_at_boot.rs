//! [Date at Boot Feature](https://codeberg.org/Limine/limine-protocol/src/branch/trunk/PROTOCOL.md#date-at-boot-feature)

use crate::requests::LimineRequest;
use core::mem::MaybeUninit;
use crate::{impl_liminine_req, LimineReqId};

#[repr(C, align(8))]
pub struct DateAtBootRequest{
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>
}

impl DateAtBootRequest {
    pub const fn new(revision: u64) -> Self {
        Self {
            id: LimineReqId::new([0x502746e184c088aa, 0xfbc5ec83e6327893]),
            revision,
            resp: MaybeUninit::uninit()
        }
    }
}

impl_liminine_req!(DateAtBootRequest, DateAtBootResponse);

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct DateAtBootResponse {
    revision: u64,
    ///Unix timestamp
    pub timestamp: u64
}