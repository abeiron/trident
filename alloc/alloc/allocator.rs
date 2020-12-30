//! `Allocator` type implementation.

pub use self::heap::{init_heap, HEAP};
pub use self::layout::Layout;

use self::linked_list::LinkedListAllocator;

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr::{read_unaligned, write_unaligned, NonNull};
use linked_list_allocator::LockedHeap;

pub mod global;
pub mod heap;
pub mod layout;
pub mod linked_list;
pub mod slab;

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

/// A wrapper around spin::Mutex to permit trait implementations.
pub struct Locked<A>
{
  inner: spin::Mutex<A>,
}

impl<A> Locked<A>
{
  pub const fn new(inner: A) -> Self
  {
    Locked {
      inner: spin::Mutex::new(inner),
    }
  }

  pub fn lock(&self) -> spin::MutexGuard<A>
  {
    self.inner.lock()
  }
}

/// Align the given address 'addr' upwards to alignment 'align'.
#[inline]
pub fn align_up(addr: usize, align: usize) -> usize
{
  let remainder = addr % align;
  if remainder == 0 {
    addr // addr already aligned
  } else {
    addr - remainder + align
  }
}

pub unsafe trait GlobalAlloc
{
  unsafe fn alloc(&self, layout: Layout) -> Option<NonNull<c_void>>;
  unsafe fn dealloc(&self, ptr: *mut c_void);
  unsafe fn realloc(
    &self,
    ptr: *mut c_void,
    old_size: usize,
    layout: Layout,
  ) -> Option<NonNull<c_void>>;

  unsafe fn alloc_aligned(&self, layout: Layout) -> Option<NonNull<c_void>>
  {
    let actual_size = layout.size + layout.align - 1 + size_of::<usize>();

    let ptr = match self.alloc(Layout::new(actual_size)) {
      Some(p) => p as usize,
      None => return None,
    };

    let aligned_ptr = layout.align_up(ptr + size_of::<usize>());
    let actual_ptr_ptr = aligned_ptr - size_of::<usize>();

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
