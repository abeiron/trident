//! `Allocator` type implementation.

pub use self::layout::Layout;
use core::ffi::c_void;
use core::mem::size_of;
use core::ptr::{NonNull, read_unaligned, write_unaligned};

pub mod global;
pub mod heap;
pub mod layout;
pub mod linked_list;
pub mod slab;

pub unsafe trait GlobalAlloc
{
  unsafe fn alloc(&self, layout: Layout) -> Option<NonNull<c_void>>;
  unsafe fn dealloc(&self, ptr: *mut c_void);

  unsafe fn alloc_aligned(&self, layout: Layout) -> Option<NonNull<c_void>>
  {
    let actual_size = layout.size + layout.align - 1 + size_of::<usize>();

    let ptr = match self.alloc(Layout::new(actual_size))
    {
      Some(p) => p as usize,
      None => return None,
    };

    let aligned_ptr = layout.align_up(ptr + size_of::<usize>());
    let actual_ptr_ptr  = aligned_ptr - size_of::<usize>();

    write_unaligned(actual_ptr_ptr as *mut usize, ptr);



    Some(NonNull::new(aligned_ptr as *mut c_void).unwrap())
  }

  unsafe fn dealloc_aligned(&self, ptr: *mut c_void)
  {
    let aligned_ptr = ptr as usize;
    let actual_ptr_ptr = aligned_ptr - size_of::<usize>();
    let actual_ptr = read_unaligned(actual_ptr_ptr as *const usize);

    self.dealloc(actual_ptr as *mut c_void);
  }
}
