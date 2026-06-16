use ash::{VkResult, vk::Result};

pub trait PresentResultExt<T> {
    fn unwrap_out_of_date(self) -> Option<T>;
}

impl<T> PresentResultExt<T> for VkResult<T> {
    fn unwrap_out_of_date(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(Result::ERROR_OUT_OF_DATE_KHR) => None,
            Err(err) => panic!("unwrap: {}", err),
        }
    }
}
