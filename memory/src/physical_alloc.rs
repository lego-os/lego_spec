use crate::AllocError;
use core::alloc::Layout;
pub unsafe trait PhysicalPageAllocator {
    fn total_size(&self) -> usize;
    fn free_size(&self) -> usize;
    fn allocated_size(&self) -> usize;
    unsafe fn alloc_pages(&mut self, layout: Layout) -> Result<*mut u8, AllocError>;
    unsafe fn free_pages(&mut self, ptr: *mut u8, layout: Layout) -> Result<(), AllocError>;
}
