use std::sync::Arc;

use ash::{Entry, LoadingError};

pub struct ThLibrary {
    pub entry: Entry,
}

impl ThLibrary {
    pub fn load() -> Result<Arc<Self>, LoadingError> {
        unsafe { Entry::load().map(|entry| Arc::new(ThLibrary { entry })) }
    }
}
