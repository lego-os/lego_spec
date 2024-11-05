#[cfg(any(feature = "rv_spv", feature = "rv_mch", feature = "rv_user"))]
use core::arch::asm;

#[cfg(any(feature = "rv_spv", feature = "rv_mch"))]
pub fn ecall() {}

#[cfg(any(feature = "rv_spv", feature = "rv_mch"))]
pub fn ebreak() {}

#[cfg(any(feature = "rv_spv", feature = "rv_mch"))]
#[inline]
pub fn wfi() {
    unsafe {
        asm!("wfi");
    }
}

#[cfg(any(feature = "rv_spv", feature = "rv_mch", feature = "rv_user"))]
#[inline]
pub fn read_tp() -> u64 {
    let mut res: u64;
    unsafe {
        asm!("mv {0},tp",out(reg) res);
    }
    res
}

#[cfg(any(feature = "rv_spv", feature = "rv_mch", feature = "rv_user"))]
#[inline]
pub fn write_tp(val: u64) {
    unsafe {
        asm!("mv tp,{0}",in(reg) val);
    }
}

#[cfg(any(feature = "rv_spv", feature = "rv_mch", feature = "rv_user"))]
#[inline]
pub fn read_sp() -> u64 {
    let mut res: u64;
    unsafe {
        asm!("mv {0},sp",out(reg) res);
    }
    res
}

#[cfg(any(feature = "rv_spv", feature = "rv_mch", feature = "rv_user"))]
#[inline]
pub fn write_sp(val: u64) {
    unsafe {
        asm!("mv sp,{}",in(reg) val);
    }
}

#[cfg(any(feature = "rv_spv", feature = "rv_mch", feature = "rv_user"))]
#[inline]
pub fn read_ra() -> u64 {
    let mut res: u64;
    unsafe {
        asm!("mv {0},ra",out(reg) res);
    }
    res
}

#[cfg(feature = "rv_spv")]
#[inline]
pub fn sfence_vma() {
    unsafe {
        asm!("sfence.vma zero, zero");
    }
}
#[cfg(feature = "rv_mch")]
#[inline]
pub fn mret() {
    unsafe {
        asm!("mret");
    }
}

#[cfg(feature = "rv_spv")]
#[inline]
pub fn sret() {
    unsafe {
        asm!("sret");
    }
}
