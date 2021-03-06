//! Implements a "global" allocator.

use super::{Allocator, Layout, HEAP};
use core::cmp::{min};
use core::ffi::c_void;
use core::ptr::{self, NonNull};
use crate::alloc::page::PAGE_SIZE;

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn __rust_allocate(size: usize, align: usize) -> *mut c_void
{
  unsafe {
    HEAP
        .lock()
        .as_mut()
        .expect("must initialize heap before calling!")
        .allocate(size, align)
  }
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn __rust_deallocate(ptr: *mut c_void, old_size: usize, align: usize)
{
  unsafe {
    HEAP
        .lock()
        .as_mut()
        .expect("must initialise heap before calling!")
        .deallocate(ptr, old_size, align)
  }
}

/// Attempt to resize an existing block of memory, preserving as much data
/// as possible.  For now, we always just allocate new memory, copy data,
/// and deallocate the old memory.
#[no_mangle]
pub extern "C" fn __rust_reallocate(
  ptr: *mut c_void,
  old_size: usize,
  size: usize,
  align: usize,
) -> *mut c_void
{
  let new_ptr = __rust_allocate(size, align);
  if new_ptr.is_null() {
    return new_ptr;
  } else {
    unsafe {
      ptr::copy(ptr, new_ptr, min(size, old_size));
    }
    __rust_deallocate(ptr, old_size, align);
    new_ptr
  }
}

/// We do not support in-place reallocation, so just return `old_size`.
///
/// :shrug:
#[no_mangle]
pub extern "C" fn __rust_reallocate_inplace(
  _ptr: *mut c_void,
  old_size: usize,
  _size: usize,
  _align: usize,
) -> usize
{
  old_size
}

/// I have no idea what this actually does, but we're supposed to have one,
/// and the other backends to implement it as something equivalent to the
/// following.
///
/// :shrug:
#[no_mangle]
pub extern "C" fn __rust_usable_size(size: usize, _align: usize) -> usize
{
  size
}

#[derive(Copy, Clone)]
pub struct Global;

unsafe impl Allocator for Global
{
  unsafe fn alloc(&self, layout: Layout) -> Option<NonNull<c_void>>
  {
    Some(NonNull::new(__rust_allocate(layout.size, layout.align)).unwrap())
  }

  unsafe fn dealloc(&self, ptr: *mut c_void, layout: Layout)
  {
    __rust_deallocate(ptr, layout.size, layout.align)
  }

  unsafe fn realloc(&self, ptr: *mut c_void, old_size: usize, layout: Layout) -> Option<NonNull<c_void>>
  {
    Some(NonNull::new(__rust_reallocate(ptr, old_size, layout.size, layout.align)).unwrap())
  }

  unsafe fn zalloc(&self, layout: Layout) -> Option<NonNull<c_void>>
  {
    // Allocate and zero a page.
    // First, let's get the allocation.
    let ret = self.alloc(layout);
    if let Some(a) = ret {
      let size = (PAGE_SIZE * pages) / 8;
      let big_ptr = ret as *mut u64;
      for i in 0..size {
        // We use big_ptr so that we can force a
        // sd (store doubleword) instruction rather than
        // the sb. This means 8 times fewer stores than before.
        // Typically, we have to be concerned about remaining
        // bytes, but fortunately 4096 % 8 = 0, so we won't
        // have any remaining bytes.
        unsafe {
          (*big_ptr.add(i)) = 0;
        }
      }
    }

    ret
  }
}
