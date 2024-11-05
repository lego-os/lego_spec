#![no_std]

#[cfg(target_arch = "riscv64")]
mod riscv;
#[cfg(target_arch = "x86_64")]
mod x86;
#[cfg(target_arch = "aarch64")]
mod aarch;

#[cfg(target_arch = "riscv64")]
pub use riscv::*;
// #[cfg(target_arch = "aarch64")]
// pub use aarch::*;
// #[cfg(target_arch = "x86_64")]
// pub use x86::*;