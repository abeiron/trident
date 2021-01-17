/// # Safety
///
/// We label the mmio function unsafe since
/// we will be working with raw memory. Rust cannot
/// make any guarantees when we do this.
pub unsafe fn write(addr: usize, offset: usize, val: u8)
{
  // Set the pointer based off of the address
  let reg = addr as *mut u8;

  // write_volatile is a member of the *mut raw
	// and we can use the .add() to give us another pointer
	// at an offset based on the original pointer's memory
	// address. NOTE: The add uses pointer arithmetic so it is
	// new_pointer = old_pointer + sizeof(pointer_type) * offset
	reg.add(offset).write_volatile(val);
}

/// # Safety
///
/// We label the mmio function unsafe since
/// we will be working with raw memory. Rust cannot
/// make any guarantees when we do this.
pub unsafe fn read(addr: usize, offset: usize, _val: u8) -> u8
{
  	// Set the pointer based off of the address
	let reg = addr as *mut u8;

	// read_volatile() is much like write_volatile() except it
	// will grab 8-bits from the pointer and give that value to us.
	// We don't add a semi-colon at the end here so that the value
	// is "returned".
	reg.add(offset).read_volatile()
}
