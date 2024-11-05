use core::panic::{RefUnwindSafe, UnwindSafe};

pub trait File:Send+Sync+RefUnwindSafe+UnwindSafe{

}