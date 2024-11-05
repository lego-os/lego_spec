#[cfg(feature = "rv_dbg")]
pub mod debug;
#[cfg(feature = "rv_hpv")]
pub mod hypervisor;
pub mod trap;

#[cfg(feature = "rv_mch")]
pub mod machine;
#[cfg(feature = "rv_spv")]
pub mod supervisor;
#[cfg(feature = "rv_user")]
pub mod user;
#[cfg(feature = "rv_virt")]
pub mod virt;

#[cfg(any(feature = "rv_spv", feature = "rv_mch", feature = "rv_user"))]
mod asm;
#[cfg(any(
    feature = "rv_spv",
    feature = "rv_mch",
    feature = "rv_dbg",
    feature = "rv_hpv",
    feature = "rv_virt"
))]
#[cfg(any(feature = "rv_spv", feature = "rv_mch", feature = "rv_user"))]
pub use asm::*;
#[cfg(any(feature = "rv_spv", feature = "rv_mch", feature = "rv_user"))]
mod macros;
#[cfg(feature = "rv_time")]
mod mmap_time;
#[cfg(feature = "rv_time")]
pub use mmap_time::*;

#[cfg(feature = "rv_dbg")]
pub use debug::*;
#[cfg(feature = "rv_hpv")]
pub use hypervisor::*;
#[cfg(feature = "rv_mch")]
pub use machine::*;
#[cfg(feature = "rv_spv")]
pub use supervisor::*;
#[cfg(feature = "rv_user")]
pub use user::*;
#[cfg(feature = "rv_virt")]
pub use virt::*;

pub const MODE_U: u64 = 0;
pub const MODE_S: u64 = 0x01;
pub const MODE_M: u64 = 0x11;
