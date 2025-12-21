# Limine Protocol for Rust
[![Crates.io](https://img.shields.io/crates/v/limine-protocol-for-rust)](https://crates.io/crates/limine-protocol-for-rust)
[![docs.rs](https://img.shields.io/badge/docs.rs-documentation-green.svg)](https://docs.rs/limine-protocol-for-rust)  
This crate implements a couple of utilities for making something compatible with the Limine Boot Protocol.
It serves as an equivalent to the *'limine.h'* file, along with some extra utilities for making response retrieval and reading provided data easier.

For more information read [The Limine Boot Protocol](https://codeberg.org/Limine/limine-protocol/src/branch/trunk/PROTOCOL.md).

Example Usage:
```rust
const REVISION: u64 = 4;

#[used]
#[unsafe(link_section = ".limine_reqs")]
static LIMINE_BASE_REVISION: [u64; 4] = use_base_revision(REVISION);

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
    kprint!("Bootloader: {} {}",bootloader_info_resp.get_name(), bootloader_info_resp.get_version());

    let memory_map_resp = MEMORY_MAP_REQUEST.get_response().expect("Memory map request had no response");
    let memory_map = memory_map_resp.get_entries();
}
```
