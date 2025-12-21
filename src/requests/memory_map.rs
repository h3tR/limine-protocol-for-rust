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
pub struct MemoryMapResponse {
    revision: u64,
    entry_count: u64,
    entries: *const *const MemoryMapEntry
}

impl MemoryMapResponse {
    pub fn get_entries(&self) -> PointerSlice<MemoryMapEntry> {
        PointerSlice::from(unsafe {
            from_raw_parts(self.entries, self.entry_count as usize)
        })
    }
}

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct MemoryMapEntry {
    base: u64,
    length: u64,
    memmap_type: u64
}

impl MemoryMapEntry {
    pub fn get_type(&self) -> MemoryMapType {
        MemoryMapType::from(self.memmap_type)
    }
}

pub enum MemoryMapType {
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

impl From<u64> for MemoryMapType {
    fn from(value: u64) -> Self {
        match value {
            0 => MemoryMapType::Usable,
            1 => MemoryMapType::Reserved,
            2 => MemoryMapType::AcpiReclaimable,
            3 => MemoryMapType::AcpiNvs,
            4 => MemoryMapType::BadMemory,
            5 => MemoryMapType::BootloaderReclaimable,
            6 => MemoryMapType::ExecutableAndModules,
            7 => MemoryMapType::Framebuffer,
            8 => MemoryMapType::AcpiTables,
            _ => panic!("Obtained invalid MemoryMap Type")
        }
    }
}