#![no_std]
#![feature(error_generic_member_access)]
#[cfg(feature = "block")]
mod block_dev;
#[cfg(feature = "bus")]
mod bus;
#[cfg(feature = "char")]
mod char_dev;
#[cfg(any(feature = "char", feature = "net", feature = "block"))]
mod dev;
#[cfg(any(feature = "char", feature = "net", feature = "block", feature = "bus"))]
mod err;
#[cfg(feature = "net")]
mod net_dev;
mod register;
#[cfg(feature = "block")]
pub use block_dev::*;
#[cfg(feature = "char")]
pub use char_dev::*;
#[cfg(any(feature = "char", feature = "net", feature = "block"))]
pub use dev::*;
#[cfg(any(feature = "char", feature = "net", feature = "block", feature = "bus"))]
pub use err::DeviceError;
pub use register::*;
