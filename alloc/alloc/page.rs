use core::cmp::min;
use core::ffi::c_void;
use core::ptr::{NonNull, null_mut};

use crate::{HEAP_SIZE, HEAP_START, Layout};
use crate::alloc::Allocator;

pub mod entry;

use entry::*;

pub mod table;

use table::*;

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

impl Page {
  // If this page has been marked as the final allocation,
  // this function returns true. Otherwise, it returns false.
  pub fn is_last(&self) -> bool {
    self.flags & PageBits::Last.val() != 0
  }

  // If the page is marked as being taken (allocated), then
  // this function returns true. Otherwise, it returns false.
  pub fn is_taken(&self) -> bool {
    self.flags & PageBits::Taken.val() != 0
  }

  // This is the opposite of is_taken().
  pub fn is_free(&self) -> bool {
    !self.is_taken()
  }

  // Clear the Page structure and all associated allocations.
  pub fn clear(&mut self) {
    self.flags = PageBits::Empty.val();
  }

  // Set a certain flag. We ran into trouble here since PageBits
  // is an enumeration and we haven't implemented the BitOr Trait
  // on it.
  pub fn set_flag(&mut self, flag: PageBits) {
    self.flags |= flag.val();
  }

  pub fn clear_flag(&mut self, flag: PageBits) {
    self.flags &= !(flag.val());
  }
}


#[derive(Copy, Clone)]
pub struct PageAlloc;

impl PageAlloc
{
  /// Map a virtual address to a physical address using 4096-byte page
  /// size.
  /// root: a mutable reference to the root Table
  /// vaddr: The virtual address to map
  /// paddr: The physical address to map
  /// bits: An OR'd bitset containing the bits the leaf should have.
  ///       The bits should contain only the following:
  ///          Read, Write, Execute, User, and/or Global
  ///       The bits MUST include one or more of the following:
  ///          Read, Write, Execute
  ///       The valid bit automatically gets added.
  pub fn map(
    &self,
    root: &mut Table,
    vaddr: usize,
    paddr: usize,
    bits: usize,
    level: usize)
  {
    // Make sure that Read, Write, or Execute have been provided
    // otherwise, we'll leak memory and always create a page fault.
    assert!(bits & 0xe != 0);
    // Extract out each VPN from the virtual address
    // On the virtual address, each VPN is exactly 9 bits,
    // which is why we use the mask 0x1ff = 0b1_1111_1111 (9 bits)
    let vpn = [
      // VPN[0] = vaddr[20:12]
      (vaddr >> 12) & 0x1ff,
      // VPN[1] = vaddr[29:21]
      (vaddr >> 21) & 0x1ff,
      // VPN[2] = vaddr[38:30]
      (vaddr >> 30) & 0x1ff,
    ];

    // Just like the virtual address, extract the physical address
    // numbers (PPN). However, PPN[2] is different in that it stores
    // 26 bits instead of 9. Therefore, we use,
    // 0x3ff_ffff = 0b11_1111_1111_1111_1111_1111_1111 (26 bits).
    let ppn = [
      // PPN[0] = paddr[20:12]
      (paddr >> 12) & 0x1ff,
      // PPN[1] = paddr[29:21]
      (paddr >> 21) & 0x1ff,
      // PPN[2] = paddr[55:30]
      (paddr >> 30) & 0x3ff_ffff,
    ];
    // We will use this as a floating reference so that we can set
    // individual entries as we walk the table.
    let mut v = &mut root.entries[vpn[2]];
    // Now, we're going to traverse the page table and set the bits
    // properly. We expect the root to be valid, however we're required to
    // create anything beyond the root.
    // In Rust, we create a range iterator using the .. operator.
    // The .rev() will reverse the iteration since we need to start with
    // VPN[2] The .. operator is inclusive on start but exclusive on end.
    // So, (0..2) will iterate 0 and 1.
    for i in (level..2).rev() {
      if !v.is_valid() {
        // Allocate a page
        let page = unsafe { self.zalloc(Layout::new(1)).unwrap() };
        // The page is already aligned by 4,096, so store it
        // directly The page is stored in the entry shifted
        // right by 2 places.
        v.set_entry(
          (page as usize >> 2)
              | EntryBits::Valid.val(),
        );
      }
      let entry = ((v.get_entry() & !0x3ff) << 2) as *mut Entry;
      v = unsafe { entry.add(vpn[i]).as_mut().unwrap() };
    }
    // When we get here, we should be at VPN[0] and v should be pointing to
    // our entry.
    // The entry structure is Figure 4.18 in the RISC-V Privileged
    // Specification
    let entry = (ppn[2] << 28) |   // PPN[2] = [53:28]
        (ppn[1] << 19) |   // PPN[1] = [27:19]
        (ppn[0] << 10) |   // PPN[0] = [18:10]
        bits |                    // Specified bits, such as User, Read, Write, etc
        EntryBits::Valid.val() |  // Valid bit
        EntryBits::Dirty.val() |  // Some machines require this to =1
        EntryBits::Access.val()   // Just like dirty, some machines require this
        ;
    // Set the entry. V should be set to the correct pointer by the loop
    // above.
    v.set_entry(entry);
  }

  /// Unmaps and frees all memory associated with a table.
  /// root: The root table to start freeing.
  /// NOTE: This does NOT free root directly. This must be
  /// freed manually.
  /// The reason we don't free the root is because it is
  /// usually embedded into the Process structure.
  pub fn unmap(&self, root: &mut Table) {
    unsafe {
      // Start with level 2
      for lv2 in 0..Table::len() {
        let ref entry_lv2 = root.entries[lv2];
        if entry_lv2.is_valid() && entry_lv2.is_branch() {
          // This is a valid entry, so drill down and free.
          let memaddr_lv1 = (entry_lv2.get_entry() & !0x3ff) << 2;
          let table_lv1 = unsafe {
            // Make table_lv1 a mutable reference instead of
            // a pointer.
            (memaddr_lv1 as *mut Table).as_mut().unwrap()
          };
          for lv1 in 0..Table::len() {
            let ref entry_lv1 = table_lv1.entries[lv1];
            if entry_lv1.is_valid() && entry_lv1.is_branch()
            {
              // The next level is level 0, which
              // cannot have branches, therefore,
              // we free here.
              let memaddr_lv0 = (entry_lv1.get_entry()
                  & !0x3ff) << 2;

              self.dealloc(memaddr_lv0 as *mut c_void, Layout::new(entry_lv1.len()));
            }
          }
          self.dealloc(memaddr_lv1 as *mut c_void, Layout::new(entry_lv2.len()));
        }
      }
    }
  }

  /// Walk the page table to convert a virtual address to a
  /// physical address.
  /// If a page fault would occur, this returns None
  /// Otherwise, it returns Some with the physical address.
  pub fn virt_to_phys(root: &Table, vaddr: usize) -> Option<usize> {
    // Walk the page table pointed to by root
    let vpn = [
      // VPN[0] = vaddr[20:12]
      (vaddr >> 12) & 0x1ff,
      // VPN[1] = vaddr[29:21]
      (vaddr >> 21) & 0x1ff,
      // VPN[2] = vaddr[38:30]
      (vaddr >> 30) & 0x1ff,
    ];

    let mut v = &root.entries[vpn[2]];
    for i in (0..=2).rev() {
      if v.is_invalid() {
        // This is an invalid entry, page fault.
        break;
      } else if v.is_leaf() {
        // According to RISC-V, a leaf can be at any level.

        // The offset mask masks off the PPN. Each PPN is 9
        // bits and they start at bit #12. So, our formula
        // 12 + i * 9
        let off_mask = (1 << (12 + i * 9)) - 1;
        let vaddr_pgoff = vaddr & off_mask;
        let addr = ((v.get_entry() << 2) as usize) & !off_mask;
        return Some(addr | vaddr_pgoff);
      }
      // Set v to the next entry which is pointed to by this
      // entry. However, the address was shifted right by 2 places
      // when stored in the page table entry, so we shift it left
      // to get it back into place.
      let entry = ((v.get_entry() & !0x3ff) << 2) as *const Entry;
      // We do i - 1 here, however we should get None or Some() above
      // before we do 0 - 1 = -1.
      v = unsafe { entry.add(vpn[i - 1]).as_ref().unwrap() };
    }

    // If we get here, we've exhausted all valid tables and haven't
    // found a leaf.
    None
  }
}

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
    for i in 0..num_pages - layout.size {
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

        return Some(NonNull::new_unchecked((ALLOC_START + PAGE_SIZE * i) as *mut c_void));
      }
    }

    // If we get here, it means that no contiguous allocation was found.
    None
  }

  unsafe fn dealloc(&self, ptr: *mut c_void, layout: Layout)
  {
    // Make sure we don't try to free a null pointer.
    assert!(!ptr.is_null());
    unsafe {
      let addr =
          HEAP_START + (ptr as usize - ALLOC_START) / PAGE_SIZE;
      // Make sure that the address makes sense. The address we
      // calculate here is the page structure, not the HEAP address!
      assert!(addr >= HEAP_START && addr < ALLOC_START);
      let mut p = addr as *mut Page;
      // println!("PTR in is {:p}, addr is 0x{:x}", ptr, addr);
      assert!((*p).is_taken(), "Freeing a non-taken page?");
      // Keep clearing pages until we hit the last page.
      while (*p).is_taken() && !(*p).is_last() {
        (*p).clear();
        p = p.add(1);
      }
      // If the following assertion fails, it is most likely
      // caused by a double-free.
      assert!(
        (*p).is_last() == true,
        "Possible double-free detected! (Not taken found \
		         before last)"
      );
      // If we get here, we've taken care of all previous pages and
      // we are on the last page.
      (*p).clear();
    }
  }

  unsafe fn realloc(&self, ptr: *mut c_void, old_size: usize, layout: Layout) -> Option<NonNull<c_void>>
  {
    let new_ptr = self.alloc(layout);
    if new_ptr.is_none() {
      return new_ptr;
    } else {
      unsafe {
        ptr::copy(ptr, new_ptr, min(layout.size, old_size));
      }

      self.dealloc(ptr, layout);
      new_ptr
    }
  }

  unsafe fn zalloc(&self, layout: Layout) -> Option<NonNull<c_void>>
  {
    // Allocate and zero a page.
    // First, let's get the allocation.
    let ret = self.alloc(layout);
    if let Some(a) = ret {
      let size = (PAGE_SIZE * layout.size) / 8;
      let big_ptr = ret.unwrap() as *mut u64;
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
