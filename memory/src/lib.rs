#![no_std]
#![feature(error_generic_member_access)]
mod err;
#[cfg(feature = "vmp")]
mod vmmap;
#[cfg(feature = "phy")]
mod physical_alloc;
#[cfg(feature = "virt")]
mod virtual_alloc;
pub use err::AllocError;
#[cfg(feature = "phy")]
pub use physical_alloc::PhysicalPageAllocator;
#[cfg(feature = "virt")]
pub use virtual_alloc::VirtualAllocator;
