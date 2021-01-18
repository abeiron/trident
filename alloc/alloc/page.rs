use core::ffi::c_void;
use std::ptr::NonNull;

use crate::{HEAP_SIZE, HEAP_START, Layout};
use crate::alloc::Allocator;

pub mod entry;
pub mod table;

static mut ALLOC_START: usize = 0;
pub const PAGE_SIZE: usize = 1 << 12;
const PAGE_ORDER: usize = 12;

/// These are the page flags, represented
/// as a u8 since the Page stores this flag.
#[repr(u8)]
pub enum PageBits
{
  Empty = 0,
  Taken = 1 << 0,
  Last = 1 << 1,
}

pub struct Page
{
  flags: u8,
}

#[derive(Copy, Clone)]
pub struct PageAlloc;

unsafe impl Allocator for PageAlloc
{
  unsafe fn alloc(&self, layout: Layout) -> Option<NonNull<c_void>>
  {
    // Need to find a contiguous allocation of pages.
    assert!(layout.size > 0);
    // We create a Page structure for each page on the heap.
    // We actually might have more since HEAP_SIZE moves as
    // well as the size of our structure, but we'll only waste a few bytes.
    let num_pages = HEAP_SIZE / PAGE_SIZE;
    let ptr = HEAP_START as *mut Page;
    for i in 0..num_pages - pages {
      let mut found = false;
      // Check to see if this Page is free.
      // If so, we have our first candidate memory address.
      if (*ptr.add(i)).is_free() {
        // It was free!
        found = true;
        for j in i..i + layout.size {
          // Now check to see if we have a contiguous allocation
          // for all of the requested pages.
          // If not, we should check elsewhere.
          if (*ptr.add(j)).is_taken() {
            found = false;
            break;
          }
        }
      }
      // We've checked to see if there are enough contiguous
      // pages to form what we need.
      // If we couldn't, 'found' will be false, otherwise it will be true.
      // Which means that we've found valid memory that we can allocate.
      if found {
        for k in i..i + layout.size - 1 {
          (*ptr.add(k)).set_flag(PageBits::Taken);
        }

        (*ptr.add(i + layout.size - 1)).set_flag(PageBits::Taken);
        (*ptr.add(i + layout.size - 1)).set_flag(PageBits::Taken);
      }
      else {
        return None;
      }
    }



    Some(NonNull::new_unchecked((ALLOC_START + PAGE_SIZE * i) as *mut c_void))
  }

  unsafe fn dealloc(&self, ptr: *mut c_void, layout: Layout)
  {
    unimplemented!()
  }

  unsafe fn realloc(&self, ptr: *mut c_void, old_size: usize, layout: Layout) -> Option<NonNull<c_void>>
  {
    unimplemented!()
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
