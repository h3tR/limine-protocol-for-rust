/*!
This crate implements a couple of utilities for making something compatible with the Limine Boot Protocol.
It serves as an equivalent to the *'limine.h'* file, along with some extra utilities for making response retrieval and reading provided data easier.

For more information read [The Limine Boot Protocol](https://codeberg.org/Limine/limine-protocol/src/branch/trunk/PROTOCOL.md).

Example Usage:
```
* const REVISION: u64 = 4;
* 
* #[used]
* #[unsafe(link_section = ".limine_reqs")]
* static LIMINE_BASE_REVISION: [u64; 4] = use_base_revision(REVISION);
* 
* #[used]
* #[unsafe(link_section = ".limine_req_start")]
* static LIMINE_REQUEST_START_MARKER: [u64; 4] = REQUEST_START_MARKER;
* 
* #[used]
* #[unsafe(link_section = ".limine_reqs")]
* pub static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new(REVISION);
* 
* #[used]
* #[unsafe(link_section = ".limine_reqs")]
* pub static HHDM_REQUEST: HigherHalfDirectMapRequest = HigherHalfDirectMapRequest::new(REVISION);
* 
* #[used]
* #[unsafe(link_section = ".limine_reqs")]
* pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new(REVISION);
* 
* #[used]
* #[unsafe(link_section = ".limine_req_end")]
* static LIMINE_REQUEST_END_MARKER: [u64; 2] = REQUEST_END_MARKER;
* 
* pub fn kernel_main() -> ! {
*     let bootloader_info_resp = BOOTLOADER_INFO_REQUEST.get_response().expect("BootloaderInfo request had no response");
*     kprint!("Bootloader: {} {}",bootloader_info_resp.get_name(), bootloader_info_resp.get_version());
* 
*     let memory_map_resp = MEMORY_MAP_REQUEST.get_response().expect("Memory map request had no response");
*     let memory_map = memory_map_resp.get_entries();
* }
* ```

*/



#![no_std]

pub mod util;
pub mod requests;

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

