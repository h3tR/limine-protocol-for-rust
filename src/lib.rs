/*!
This crate implements a couple of utilities for making something compatible with the Limine Boot Protocol.
It serves as an equivalent to the *'limine.h'* file, along with some extra utilities for making response retrieval and reading provided data easier.

For more information read [The Limine Boot Protocol](https://codeberg.org/Limine/limine-protocol/src/branch/trunk/PROTOCOL.md).

Example Usage:
```
const REVISION: u64 = 4;

#[used]
#[unsafe(link_section = ".limine_reqs")]
static LIMINE_BASE_REVISION: [u64; 4] = use_base_revision(4);

#[used]
#[unsafe(link_section = ".limine_req_start")]
static LIMINE_REQUEST_START_MARKER: [u64; 4] = REQUEST_START_MARKER;

#[used]
#[unsafe(link_section = ".limine_reqs")]
pub static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new(REVISION);

#[used]
#[unsafe(link_section = ".limine_reqs")]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new(REVISION);

#[used]
#[unsafe(link_section = ".limine_reqs")]
pub static BOOTLOADER_INFO_REQUEST: BootloaderInfoRequest = BootloaderInfoRequest::new(REVISION);

#[used]
#[unsafe(link_section = ".limine_req_end")]
static LIMINE_REQUEST_END_MARKER: [u64; 2] = REQUEST_END_MARKER;

pub fn kernel_main() -> ! {
    let bootloader_info_resp = BOOTLOADER_INFO_REQUEST.get_response().expect("BootloaderInfo request had no response");
    kprint!("Bootloader: {} {}",bootloader_info_resp.get_name(), bootloader_info_resp.get_version())

    let memory_map_resp = MEMORY_MAP_REQUEST.get_response().expect("Memory map request had no response");
    let memory_map = memory_map_resp.get_entries();
}
```

*/



#![no_std]

mod util;

use crate::util::null_terminated_string;
use core::mem::MaybeUninit;
use core::slice::from_raw_parts;

pub const REQUEST_START_MARKER: [u64; 4] = [ 0xf6b8f4b39de7d1ae, 0xfab91a6940fcb9cf, 0x785c6ed015d3e316, 0x181e920a7852b9d9 ];
pub const REQUEST_END_MARKER: [u64; 2] = [ 0xadc0e0531bb10d03, 0x9572709f31764c62 ];


///Wrapper for the Limine Base Revision magic number
pub const fn use_base_revision(revision: u64) -> [u64; 4]{
    [ 0xf9562b2d5c95a6c8, 0x6a7b384944536bdc, revision, 0 ]
}

#[repr(C, align(8))]
struct LimineReqId {
    common_magic: [u64; 2],
    other: [u64; 2]
}

impl LimineReqId {
    const fn new(other: [u64; 2]) -> Self {
        Self {
            common_magic: [0xc7b1dd30df4c8b88, 0x0a82e883a194f07b],
            other
        }
    }
}

macro_rules! gen_get_response {
    ($get:ty) => {
        pub fn get_response(&self) -> Option<&$get> {
            unsafe {
                if self.resp.assume_init() == 0 {
                   return None
                }
                (self.resp.assume_init() as *const $get).as_ref()
            }
        }
    };
}


#[repr(C, align(8))]
pub struct MemoryMapRequest {
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>
}

impl MemoryMapRequest {
    const fn new(revision: u64) -> Self {
        Self {
            id: LimineReqId::new([0x67cf3d9d378a806f, 0xe304acdfc50c3c62]),
            revision,
            resp: MaybeUninit::uninit()
        }
    }

    gen_get_response!(MemoryMapResponse);
}


#[repr(C, align(8))]
pub struct MemoryMapResponse {
    revision: u64,
    entry_count: u64,
    entries: *const MemoryMapEntry
}

impl MemoryMapResponse {
    pub fn get_entries(&self) -> &[MemoryMapEntry] {
        unsafe {
            from_raw_parts(self.entries, self.entry_count as usize)
        }
    }
}

#[repr(C, align(8))]
pub struct MemoryMapEntry {
    base: u64,
    length: u64,
    memmap_type: u64
}

impl MemoryMapEntry {
    fn type_as_enum(&self) -> MemoryMapType {
        match self.memmap_type {
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

#[repr(C, align(8))]
pub struct FramebufferRequest {
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>
}

impl FramebufferRequest {
    const fn new(revision: u64) -> Self {
        Self {
            id: LimineReqId::new([0x9d5827dcd881dd75, 0xa3148604f6fab11b]),
            revision,
            resp: MaybeUninit::uninit()
        }
    }

    gen_get_response!(FramebufferResponse);
}


#[repr(C, align(8))]
#[derive(Debug)]
pub struct FramebufferResponse {
    revision: u64,
    entry_count: u64,
    entries: *const Framebuffer
}

impl FramebufferResponse {
    pub fn get_entries(&self) -> &[Framebuffer] {
        unsafe {
            from_raw_parts(self.entries, self.entry_count as usize)
        }
    }
}

#[repr(C, align(8))]
#[derive(Debug)]
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
    pub mode_count: u64,
    modes: *const VideoMode
}

impl Framebuffer {
    pub fn get_modes(&self) -> &[VideoMode] {
        unsafe {
            from_raw_parts(self.modes, self.mode_count as usize)
        }
    }
}

#[repr(C, align(8))]
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

#[repr(C, align(8))]
pub struct BootloaderInfoRequest{
    id: LimineReqId,
    revision: u64,
    resp: MaybeUninit<usize>
}

impl BootloaderInfoRequest {
    const fn new(revision: u64) -> Self {
        Self {
            id: LimineReqId::new([0xf55038d8e2a1202f, 0x279426fcf5f59740]),
            revision,
            resp: MaybeUninit::uninit()
        }
    }

    gen_get_response!(BootloaderInfoResponse);
}

#[repr(C, align(8))]
pub struct BootloaderInfoResponse {
    revision: u64,
    name: *const u8,
    version: *const u8
}

impl BootloaderInfoResponse {
    pub fn get_name(&self) -> &str {
        null_terminated_string(self.name).unwrap()
    }

    pub fn get_version(&self) -> &str {
        null_terminated_string(self.version).unwrap()
    }
}

