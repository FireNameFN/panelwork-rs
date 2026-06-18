use ash::{VkResult, vk::Result};

pub trait SwapchainResultExt<T> {
    fn unwrap_out_of_date(self) -> Option<T>;
}

impl<T> SwapchainResultExt<T> for VkResult<T> {
    fn unwrap_out_of_date(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(Result::ERROR_OUT_OF_DATE_KHR) => None,
            Err(err) => panic!("unwrap: {}", err),
        }
    }
}
