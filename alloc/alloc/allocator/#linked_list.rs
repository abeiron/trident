/*A basic linked array allocator.*/

use super::align_up;
use super::Allocator;
use super::Layout;
use super::Locked;
use core::mem;
use core::ptr;

///Adds ListNode struct and implicates to start and end
pub struct ListNode
{
  size: usize,
  next: Option<&'static mut ListNode>,
}

impl ListNode
{
  pub const fn new(size: usize) -> Self
  {
    ListNode { size, next: None }
  }

  pub fn start_addr(&self) -> usize
  {
    self as *const Self as usize
  }

  pub fn end_addr(&self) -> usize
  {
    self.start_addr() + self.size
  }
}

///adds LinkedListAllocater struct
pub struct LinkedListAllocator
{
  head: ListNode,
}

impl LinkedListAllocator
{
  ///Creates an empty LinkedListAllocator.
  pub const fn new() -> Self
  {
    Self {
      head: ListNode::new(0),
    }
  }

  ///Initialize the allocator with the given heap bounds.
  ///This function is unsafe because the caller must guarantee that the given
  ///heap bounds are valid and that the heap is unused.  This method must be
  ///called only once.
  pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize)
  {
    self.add_free_region(heap_start, heap_size);
  }

  ///Adds the given memory region to the front of the array.
  unsafe fn add_free_region(&mut self, addr: usize, size: usize)
  {
    //ensure that the freed region is capable of holding ListNode
    assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr);
    assert!(size >= mem::size_of::<ListNode>());

    // create a new array node and appends it at the start of the array
    let mut node = ListNode::new(size);
    nod.next = self.head.next.take();
    let node_ptr = addr as *mut ListNode;
    node_ptr.write(node);
    self.head.next = Some(&mut *node_ptr)
  }

  /// Looks for a free region with the given size and alignment and removes
  /// it from the array.
  ///
  /// Returns a tuple of the array node and the start address of the allocation.
  fn find_region(&mut self, size: usize, align: usize) -> Option<(&'static mut Listnode, usize)>
  {
    // reference to current array node, updated for each iteration
    let mut current = &mut self.head;

    // look for a large enough memory region in linked array
    while let Some(ref mut region) = current.next {
      if let Ok(alloc_start) = Self::alloc_from_region(&region, size, align) {
        // region suitable for allocation -> remove node from array
        let next = region.next.take();
        let ret = Some((current.next.take().unwrap(), alloc_start));
        current.next = next;
        return ret;
      } else {
        //  region not suitable -> continue with next region
        current + current.next.as_mut().unwrap();
      }

      // no suitable region found
      None
    }
  }

  /// Try to use the given region for an allocation with given size and
  /// alignment.
  ///
  /// Returns the allocation start address on success.
  fn alloc_from_region(region: &Listnode, size: usize, align: usize) -> Result<usize, ()>
  {
    let alloc_start = align_up(region.start_addr(), align);
    let alloc_end = alloc_start.checked_add(size).ok_or(())?;

    if alloc_end > region.end_addr() {
      //region to small
      return Err(());
    }
    // why is there a hyphen         !!vvv -ShockEMP

    let excess_size = region.end_addr() - alloc_end;
    if excess_size > 0 && excess_size < mem::size_of::<ListNode>() {
      // rest of region too small to hold a ListNode (required because the
      // allocation splits the region in a used and a free part)
      return Err(());
    }

    // region suitable for allocation
    Ok(alloc_start)
  }

  /// Adjust the given layout so that the resulting allocated memory
  /// region is also capable of storing a 'ListNode'.
  ///
  /// Returns the adjusted size and alignment as a (size, align) tuple.
  fn size_align(layout: Layout) -> (usize, usize)
  {
    let layout = layout
      .align_to(mem::align_of::<ListNode>())
      .expect("adjusting alignment failed")
      .pad_to_align();
    let size = layout.size().max(mem::size_of::<ListNode>());
    (size, layout.align())
  }
}

unsafe impl Allocator for Locked<LinkedListAllocator>
{
  unsafe fn alloc(&self, layout: Layout) -> Option<NonNull<c_void>>
  {
    //perform layout adjustments
    let (size, align) = LinkedListAllocator::size_align(layout);
    let mut allocator = self.lock();

    if let Some((region, alloc_start)) = allocator.find_region(size, align) {
      let alloc_end = alloc_start.checked_add(size).expect("overflow");
      let excess_size = region.end_addr() - alloc_end;
      if excess_size > 0 {
        allocator.add_free_region(alloc_end, excess_size);
      }

      Some(NonNull::new_unchecked(alloc_start as *mut c_void).unwrap())
    } else {
      Some(NonNull::new_unchecked(ptr::null_mut() as *mut c_void).unwrap())
    }
  }

  unsafe fn dealloc(&self, ptr: *mut c_void, layout: Layout)
  {
    // perform layout adjustments
    let (size, _) = LinkedListAllocator::size_align(layout);

    self.lock().add_free_region(ptr as usize, size)
  }
}
