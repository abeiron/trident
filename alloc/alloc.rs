/*!
Defines the `AllocRef` trait as a framework for memory allocation.
*/

use core::cell::RefCell;
use core::ffi::c_void;
use core::mem::size_of;
use core::ptr::{NonNull, read_unaligned, write_unaligned};

use spin::{Mutex, MutexGuard};

use crate::{_heap_size, _heap_start};

pub use self::global::Global;
pub use self::heap::{HEAP, init_heap};
pub use self::layout::Layout;
use self::page::PAGE_SIZE;

pub mod comp;
pub mod ctx;
pub mod entity;
pub mod global;
pub mod heap;
pub mod layout;
pub mod page;
pub mod system;
pub mod world;


pub unsafe fn alloc_one<T>(alloc: &mut dyn AllocRef) -> Option<NonNull<u8>>
{
  alloc
      .alloc_aligned(Layout::from_type::<T>())
      .map(|ptr| ptr.cast::<T>())
}

pub unsafe fn alloc_array<T>(alloc: &mut dyn AllocRef, size: usize) -> Option<NonNull<u8>>
{
  alloc
      .alloc_aligned(Layout::from_type_array::<T>(size))
      .map(|ptr| ptr.cast::<T>())
}

/// A wrapper around spin::Mutex to permit trait implementations.
pub struct Locked<A: AllocRef>
{
  /// The inner spin::Mutex
  inner: Mutex<A>,
}

impl<A> Locked<A>
  where
      A: AllocRef
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
#[inline(always)]
pub fn align_up(addr: usize, align: usize) -> usize
{
  let remainder = addr % align;
  if remainder == 0 {
    addr // addr already aligned
  } else {
    addr - remainder + align
  }
}

/// The "AllocRef" trait.
///
/// Defines the framework for an allocator.
pub unsafe trait AllocRef
{
  /// Allocates a block of memory.
  unsafe fn alloc(&self, layout: Layout) -> Option<NonNull<u8>>;

  /// Deallocates a block of memory.
  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout);

  /// Reallocates a block of memory.
  unsafe fn realloc(
    &self,
    ptr: *mut u8,
    old_size: usize,
    layout: Layout,
  ) -> Option<NonNull<u8>>;

  /// Allocate a zero a page or multiple pages.
  ///
  /// pages: the number of pages to allocate.
  ///
  /// Each page is `PAGE_SIZE` which is calculated as 1 << `PAGE_ORDER`.
  /// On RISC-V, this is typically 4096 bytes.
  unsafe fn zalloc(&self, pages: usize) -> Option<NonNull<u8>>
  {
    let size = (PAGE_SIZE * pages) / 8;
    let ret = self.alloc(Layout::from_size(size));
    if !ret.is_none() {
      let big_ptr = ret.unwrap() as *mut u64;
      for i in 0..size {
        // We use big_ptr so that we can force an
        // sd (store doubleword) instruction rather than
        // the sb. This means 8x fewer stores than before.
        // Typically we have to be concerned about remaining
        // bytes, but fortunately 4096 % 8 = 0, so we
        // won't have any remaining bytes.
        unsafe {
          (*big_ptr.add(i)) = 0;
        }
      }
    }

    ret
  }

  /// Allocates an aligned block of memory.
  unsafe fn alloc_aligned(&self, layout: Layout) -> Option<NonNull<u8>>
  {
    let actual_size = layout.size() + layout.align() - 1 + size_of::<usize>();

    let ptr = match self.alloc(Layout::from_size(actual_size)) {
      Some(p) => p.as_ptr() as usize,
      None => return None,
    };

    let aligned_ptr = layout.align_up(ptr + size_of::<usize>());
    let actual_ptr_ptr = aligned_ptr - size_of::<usize>();

    write_unaligned(actual_ptr_ptr as *mut usize, ptr);

    Some(NonNull::new_unchecked(aligned_ptr as *mut u8))
  }

  /// Deallocates an aligned block of memory.
  unsafe fn dealloc_aligned(&self, ptr: *mut u8, layout: Layout)
  {
    let aligned_ptr = ptr as usize;
    let actual_ptr_ptr = aligned_ptr - size_of::<usize>();
    let actual_ptr = read_unaligned(actual_ptr_ptr as *const usize);

    self.dealloc(actual_ptr as *mut u8, layout);
  }
}

/// Implemented to allow the allocator to be put into a borrowed RefCell.
unsafe impl<A: AllocRef> AllocRef for &RefCell<A>
{
  unsafe fn alloc(&self, layout: Layout) -> Option<NonNull<u8>>
  {
    self.borrow_mut().alloc(layout)
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
  {
    self.borrow_mut().dealloc(ptr, layout)
  }

  unsafe fn realloc(&self, ptr: *mut u8, old_size: usize, layout: Layout) -> Option<NonNull<u8>>
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
unsafe impl<A: AllocRef> AllocRef for Mutex<A>
{
  unsafe fn alloc(&self, layout: Layout) -> Option<NonNull<u8>>
  {
    self.lock().alloc(layout)
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
  {
    self.lock().dealloc(ptr, layout)
  }

  unsafe fn realloc(&self, ptr: *mut u8, old_size: usize, layout: Layout) -> Option<NonNull<u8>>
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
unsafe impl<A: AllocRef> AllocRef for Locked<A>
{
  unsafe fn alloc(&self, layout: Layout) -> Option<NonNull<u8>>
  {
    self.lock().alloc(layout)
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
  {
    self.lock().dealloc(ptr, layout)
  }

  unsafe fn realloc(&self, ptr: *mut u8, old_size: usize, layout: Layout) -> Option<NonNull<u8>>
  {
    self.lock().realloc(ptr, old_size, layout)
  }
}

