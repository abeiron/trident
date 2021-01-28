//! Implements an allocator-aware smart pointer called `Unq`.

use crate::{
  alloc::{alloc_one, Allocator, Global},
  borrow::{Borrow, BorrowMut},
  convert::{AsMut, AsRef},
  marker::{PhantomData, Unsize},
  pin::Pin,
};

use core::ops::{CoerceUnsized, Deref, DerefMut};
use core::ptr::{drop_in_place, write, NonNull};

/// An allocator-aware smart pointer most similar to C++'s `unique_ptr`.
/// 
/// # Usage
/// 
/// ```no_compile
/// let ptr = Unq::new(100_000);
/// let num = *ptr;
/// ```
/// 
/// TODO: Write better documentation.
pub struct Unq<T: ?Sized, A: Allocator = Global>
{
  ptr: NonNull<T>,
  alloc: A,
  _ghost: PhantomData<T>,
}

impl<T, A: Allocator> Unq<T, A>
{
  pub fn new_with(val: T, mut alloc: A) -> Self
  {
    let mut ptr = unsafe { alloc_one::<T>(&mut alloc).expect("allocation error") };
    
    unsafe {
      write(ptr.as_mut(), val);
    }
    
    Self {
      ptr,
      alloc,
      _ghost: PhantomData,
    }
  }
  
  pub fn pin_with(val: T, alloc: A) -> Pin<Self>
  {
    unsafe { Pin::new_unchecked(Self::new_with(val, alloc)) }
  }
}

impl<T: ?Sized, A: Allocator> Unq<T, A>
{
  pub unsafe fn from_raw_with(ptr: NonNull<T>, alloc: A) -> Self
  {
    Self {
      ptr,
      alloc,
      _ghost: PhantomData,
    }
  }
  
  pub fn leak<'a>(unq: Unq<T, A>) -> &'a mut T
  where
      A: 'a
  {
    let reference = unsafe { &mut *unq.ptr.as_ptr() };
    core::mem::forget(unq);
    return reference;
  }
  
  #[inline]
  pub fn into_raw(unq: Self) -> *mut T
  {
    let ptr = unq.ptr.as_ptr();
    core::mem::forget(unq);
    return ptr;
  }
}

impl<T> Unq<T, Global>
{
  pub fn new(val: T) -> Self
  {
    Self::new_with(val, Global)
  }
  
  pub fn pin(val: T) -> Pin<Self>
  {
    Self::pin_with(val, Global)
  }
}

impl<T: ?Sized> Unq<T, Global>
{
  pub unsafe fn from_raw(ptr: NonNull<T>) -> Self
  {
    Self::from_raw_with(ptr, Global)
  }
}

impl<T: ?Sized, A: Allocator> Deref for Unq<T, A>
{
  type Target = T;
  
  #[inline]
  fn deref(&self) -> &Self::Target
  {
    unsafe { self.ptr.as_ref() }
  }
}

impl<T: ?Sized, A: Allocator> DerefMut for Unq<T, A>
{
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target
  {
    unsafe { self.ptr.as_mut() }
  }
}

impl<T: ?Sized, A: Allocator> AsRef<T> for Unq<T, A>
{
  #[inline]
  fn as_ref(&self) -> &T
  {
    self
  }
}

impl<T: ?Sized, A: Allocator> AsMut<T> for Unq<T, A>
{
  #[inline]
  fn as_mut(&mut self) -> &mut T
  {
    self
  }
}

impl<T: ?Sized, A: Allocator> Borrow<T> for Unq<T, A>
{
  #[inline]
  fn borrow(&self) -> &T
  {
    self
  }
}

impl<T: ?Sized, A: Allocator> BorrowMut<T> for Unq<T, A>
{
  #[inline]
  fn borrow_mut(&mut self) -> &mut T
  {
    self
  }
}

impl<T: ?Sized, A: Allocator> Drop for Unq<T, A>
{
  #[inline]
  fn drop(&mut self)
  {
    unsafe {
      drop_in_place(self.ptr.as_ptr());
      self.alloc.dealloc(self.ptr.cast().as_ptr());
    }
  }
}

impl<T: ?Sized + Unsize<U>, U: ?Sized, A: Allocator> CoerceUnsized<Unq<U, A>> for Unq<T, A> {}

impl<T: ?Sized, A: Allocator> Unpin for Unq<T, A> {}

