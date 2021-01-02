use core::{
  borrow::Borrow,
  hash::*,
  mem::{replace, swap},
};

use crate::{
  alloc::{Allocator, Global},
  array::Array,
  hash::SipHash,
};


