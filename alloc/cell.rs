//! Implements "cell" types.

use core::{
  cell::UnsafeCell,
  sync::atomic::{AtomicUsize, Ordering},
};
