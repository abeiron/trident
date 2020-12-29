//! `Allocator` type implementation.

pub use self::layout::Layout;
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use linked_list::LinkedListAllocator;
use linked_list_allocator::LockedHeap;
use core::ffi::c_void;
use core::mem::size_of;
<<<<<<< HEAD
use core::ptr::{NonNull, read_unaligned, write_unaligned};
use x86_64::{
    structures::paging::{
      mapper::MapToError, FrameAllocator, Mapper, page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

#[global_allocator]
static Allocator: Locked<LinkedListAllocator> =
    Locked::new(LinkedListAllocator::new());
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

=======
use core::ptr::{NonNull, read_unaligned, write_unaligned};
>>>>>>> fac3c4af15055b74932574380d89a78d5e08b5da

pub mod global;
pub mod heap;
pub mod layout;
pub mod linked_list;
pub mod slab;

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

/// A wrapper around spin::Mutex to permit trait implementations.
pub struct Locked<A> {
  inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
  pub const fn new(inner: A) -> Self {
    Locked {
      inner: spin::Mutex::new(inner),
    }
  }

  pub fn lock(&self) -> spin::MutexGuard<A> {
    sefl.inner.lock()
  }
}

/// Align the given address 'addr' upwards to alignment 'align'.
fn align_up(addr: usize, align: usize) -> usize {
  let remainder = addr % align;
  if remainder == 0 {
    addr // addr already aligned
  } else {
    addr - remainder + align
  }
}

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
  // [...] map all heap pages to physical frames
  let page_range = {
    let heap_start = VirtAddr::new(HEAP_START as u64);
    let heap_end = heap_start + HEAP_SIZE - lu64;
    let heap_start_page = Page::containing_address(heap_start);
    let heap_end_page = Page::containing_address(heap_end);
    Page::range_inclusive(heap_start_page, heap_end_page)
  };

  for page in page_range {
    let frame = frame_allocator
        .allocate_frame()
        .ok_or(MapToError::FrameAllocationFailed)?;
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    unsafe {
      mapper.map_to(page, frame, flags, frame_allocator)?.flush()
    };
  }
  // new
  unsafe {
    ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
  }

  Ok(())
}

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

