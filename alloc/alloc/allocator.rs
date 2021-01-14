//! `Allocator` type implementation.

use core::cell::RefCell;
use core::ffi::c_void;
use core::mem::size_of;
use core::ptr::{NonNull, read_unaligned, write_unaligned};

use spin::{Mutex, MutexGuard};

pub use crate::alloc::layout::Layout;

pub use self::global::Global;
pub use self::heap::{HEAP, init_heap};

pub mod global;
pub mod heap;
pub mod slab;

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub unsafe fn alloc_one<T>(alloc: &mut dyn Allocator) -> Option<NonNull<T>>
{
  alloc
      .alloc_aligned(Layout::from_type::<T>())
      .map(|ptr| ptr.cast::<T>())
}

pub unsafe fn alloc_array<T>(alloc: &mut dyn Allocator, size: usize) -> Option<NonNull<T>>
{
  alloc
      .alloc_aligned(Layout::from_type_array::<T>(size))
      .map(|ptr| ptr.cast::<T>())
}

/// A wrapper around spin::Mutex to permit trait implementations.
pub struct Locked<A: Allocator>
{
  /// The inner spin::Mutex
  inner: Mutex<A>,
}

impl<A> Locked<A>
where
    A: Allocator
{
  pub const fn new(inner: A) -> Self
  {
    Locked {
      inner: Mutex::new(inner),
    }
  }

  pub fn lock(&self) -> MutexGuard<A>
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

/// The "Allocator" trait.
///
/// Defines a framework for an allocator.
pub unsafe trait Allocator
{
  /// Allocates a block of memory.
  unsafe fn alloc(&self, layout: Layout) -> Option<NonNull<c_void>>;

  /// Deallocates a block of memory.
  unsafe fn dealloc(&self, ptr: *mut c_void, layout: Layout);

  /// Reallocates a block of memory.
  unsafe fn realloc(
    &self,
    ptr: *mut c_void,
    old_size: usize,
    layout: Layout,
  ) -> Option<NonNull<c_void>>;

  /// Allocates an aligned block of memory.
  unsafe fn alloc_aligned(&self, layout: Layout) -> Option<NonNull<c_void>>
  {
    let actual_size = layout.size + layout.align - 1 + size_of::<usize>();

    let ptr = match self.alloc(Layout::new(actual_size)) {
      Some(p) => p.as_ptr() as usize,
      None => return None,
    };

    let aligned_ptr = layout.align_up(ptr + size_of::<usize>());
    let actual_ptr_ptr = aligned_ptr - size_of::<usize>();

    write_unaligned(actual_ptr_ptr as *mut usize, ptr);

    Some(NonNull::new_unchecked(aligned_ptr as *mut c_void).unwrap())
  }

  /// Deallocates an aligned block of memory.
  unsafe fn dealloc_aligned(&self, ptr: *mut c_void, layout: Layout)
  {
    let aligned_ptr = ptr as usize;
    let actual_ptr_ptr = aligned_ptr - size_of::<usize>();
    let actual_ptr = read_unaligned(actual_ptr_ptr as *const usize);

    self.dealloc(actual_ptr as *mut c_void, layout);
  }
}

/// Implemented to allow the allocator to be put into a borrowed RefCell.
unsafe impl<A: Allocator> Allocator for &RefCell<A>
{
  unsafe fn alloc(&self, layout: Layout) -> Option<NonNull<c_void>>
  {
    self.borrow_mut().alloc(layout)
  }

  unsafe fn dealloc(&self, ptr: *mut c_void, layout: Layout)
  {
    self.borrow_mut().dealloc(ptr, layout)
  }

  unsafe fn realloc(&self, ptr: *mut c_void, old_size: usize, layout: Layout) -> Option<NonNull<c_void>>
  {
    self.borrow_mut().realloc(ptr, old_size, layout)
  }
}

/// Implemented to allow mutually exclusive instances of an allocator.
///
/// ```no_compile
/// let mut a = Mutex::new(Global)
/// a.lock().alloc(Layout::new(size))
/// ```
unsafe impl<A: Allocator> Allocator for Mutex<A>
{
  unsafe fn alloc(&self, layout: Layout) -> Option<NonNull<c_void>>
  {
    self.lock().alloc(layout)
  }

  unsafe fn dealloc(&self, ptr: *mut c_void, layout: Layout)
  {
    self.lock().dealloc(ptr, layout)
  }

  unsafe fn realloc(&self, ptr: *mut c_void, old_size: usize, layout: Layout) -> Option<NonNull<c_void>>
  {
    self.lock().realloc(ptr, old_size, layout)
  }
}

/// Alternative for a Mutex<A>
///
/// ```no_compile
/// let mut a = Locked::new(Global)
/// a.lock().alloc(Layout::new(size))
/// ```
unsafe impl<A: Allocator> Allocator for Locked<A>
{
  unsafe fn alloc(&self, layout: Layout) -> Option<NonNull<c_void>>
  {
    self.lock().alloc(layout)
  }

  unsafe fn dealloc(&self, ptr: *mut c_void, layout: Layout)
  {
    self.lock().dealloc(ptr, layout)
  }

  unsafe fn realloc(&self, ptr: *mut c_void, old_size: usize, layout: Layout) -> Option<NonNull<c_void>>
  {
    self.lock().realloc(ptr, old_size, layout)
  }
}
