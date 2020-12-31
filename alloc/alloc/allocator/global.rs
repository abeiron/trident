/*TODO: Implement a global allocator.*/

use super::{GlobalAlloc, Layout, HEAP};
use core::ffi::c_void;
use core::ptr::{read_unaligned, write_unaligned, NonNull};

#[no_mangle]
pub extern "C" fn __rust_allocate(size: usize, align: usize) -> *mut u8
{
  unsafe {
    HEAP
      .lock()
      .as_mut()
      .expect("must initialize heap before calling!")
      .allocate(size, align)
  }
}

pub extern "C" fn __rust_deallocate(ptr: *mut u8, old_size: usize, align: usize)
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
  ptr: *mut u8,
  old_size: usize,
  size: usize,
  align: usize,
) -> *mut u8
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
#[no_mangle]
pub extern "C" fn __rust_reallocate_inplace(
  _ptr: *mut u8,
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
#[no_mangle]
pub extern "C" fn __rust_usable_size(size: usize, _align: usize) -> usize
{
  size
}

#[global_allocator]
pub struct Global;

unsafe impl GlobalAlloc for Global
{
  fn alloc(&self, layout: Layout) -> Option<NonNull<c_void>>
  {
    Some(NonNull::new(__rust_allocate(layout.size, layout.align) as *mut c_void).unwrap())
  }

  fn dealloc(&self, ptr: *mut c_void, layout: Layout)
  {
    __rust_deallocate(ptr as *mut u8, layout.size, layout.align)
  }

  fn realloc(&self, ptr: *mut c_void, old_size: usize, layout: Layout) -> Option<NonNull<c_void>>
  {
    Some(
      NonNull::new(__rust_reallocate(ptr as *mut u8, old_size, layout.size, layout.align) as *mut c_void)
        .unwrap(),
    )
  }
}
