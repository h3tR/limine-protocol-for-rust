use crate::requests::LimineRequest;
use core::mem::MaybeUninit;
use core::slice::from_raw_parts;
use crate::{impl_liminine_req, LimineReqId};
use crate::util::PointerSlice;

#[repr(C, align(8))]
pub struct MemoryMapRequest {
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>
}

impl MemoryMapRequest {
    pub const fn new(revision: u64) -> Self {
        Self {
            id: LimineReqId::new([0x67cf3d9d378a806f, 0xe304acdfc50c3c62]),
            revision,
            resp: MaybeUninit::uninit()
        }
    }
}

impl_liminine_req!(MemoryMapRequest, MemoryMapResponse);

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct MemoryMapResponse {
    revision: u64,
    entry_count: u64,
    entries: *const *const MemoryRegionInfo
}

impl MemoryMapResponse {
    pub fn get_entries(&self) -> PointerSlice<MemoryRegionInfo> {
        PointerSlice::from(unsafe {
            from_raw_parts(self.entries, self.entry_count as usize)
        })
    }
}

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct MemoryRegionInfo {
    pub base: u64,
    pub length: u64,
    region_type: u64
}

impl MemoryRegionInfo {
    pub fn get_type(&self) -> MemoryRegionType {
        MemoryRegionType::from(self.region_type)
    }
}

#[derive(PartialEq, Debug)]
pub enum MemoryRegionType {
    Usable,
    Reserved,
    AcpiReclaimable,
    AcpiNvs,
    BadMemory,
    BootloaderReclaimable,
    ExecutableAndModules,
    Framebuffer,
    AcpiTables
}

impl From<u64> for MemoryRegionType {
    fn from(value: u64) -> Self {
        match value {
            0 => MemoryRegionType::Usable,
            1 => MemoryRegionType::Reserved,
            2 => MemoryRegionType::AcpiReclaimable,
            3 => MemoryRegionType::AcpiNvs,
            4 => MemoryRegionType::BadMemory,
            5 => MemoryRegionType::BootloaderReclaimable,
            6 => MemoryRegionType::ExecutableAndModules,
            7 => MemoryRegionType::Framebuffer,
            8 => MemoryRegionType::AcpiTables,
            _ => panic!("Obtained invalid MemoryMap Type")
        }
    }
}