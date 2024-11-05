use core::panic::{RefUnwindSafe, UnwindSafe};

pub trait Directory: Send + Sync + RefUnwindSafe + UnwindSafe {}
