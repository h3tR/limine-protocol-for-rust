mod memory_map;
mod framebuffer;
mod bootloader_info;

///Implements [LimineRequest] for $req. Requires \$req to have a resp field of the type core::mem::MaybeUninit\<usize\>.
#[macro_export]
macro_rules! impl_liminine_req {
    ($req:ty, $resp:ty) => {
        impl LimineRequest for $req {
            type Response = $resp;

            fn get_response(&self) -> Option<&Self::Response> {
                unsafe {
                    if self.resp.assume_init() == 0 {
                       return None
                    }
                    (self.resp.assume_init() as *const Self::Response).as_ref()
                }
            }
        }
    };
}

pub trait LimineRequest {
    type Response;
    fn get_response(&self) -> Option<&Self::Response>;
}