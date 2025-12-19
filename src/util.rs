use core::slice;
use core::str::from_utf8;

pub fn null_terminated_string(ptr: *const u8) -> Option<&'static str> {
    let mut size: usize = 0;
    //determine the size of the u8 slice
    loop {
        unsafe {
            if ptr.add(size).read() == 0 {
                break;
            }
            size += 1;
        }
    }
    let char_slice = unsafe { slice::from_raw_parts(ptr, size) };
    from_utf8(char_slice).map_or(None, |s| Some(s))
}