//! A dynamically sized array type.

use core::{
  iter::{FromIterator, IntoIterator},
  mem::needs_drop,
  ops::{Deref, DerefMut},
  ptr::{drop_in_place, read},
  slice,
};

mod raw_array;
use self::raw_array::RawArray;

mod small_array;

use crate::alloc::{Allocator, Global, Layout};

pub struct Array<T, A: Allocator = Global>
{
  size: usize,
  buf: RawArray<T, A>,
}

impl<T, A: Allocator> Array<T, A: Allocator>
{
  pub fn new_with(alloc: A) -> Self
  {
    Array {
      size: 0,
      buf: RawArray::new(alloc),
    }
  }

  pub fn resize_with<F>(&mut self, new_size: usize, f: F)
    where
        F: Fn() -> T,
  {
    if new_size < self.size && needs_drop::<T>()
    {
      for i in new_size..self.size
      {
        unsafe {
          drop_in_place(self.buf.ptr.offset(i as isize));
        }
      }
    } else if new_size > self.size
    {
      if new_size > self.buf.capacity
      {
        self.reserve(new_size);
      }

      for i in self.size..new_size
      {
        unsafe {
          self.buf.ptr.offset(i as isize).write(f());
        }
      }
    }

    self.size = new_size;
  }

  pub fn resize(&mut self, new_size: usize, value: T)
    where
        T: Clone,
  {
    self.resize_with(new_size, || value.clone());
  }

  pub fn resize_default(&mut self, new_size: usize)
    where
        T: Default,
  {
    self.resize_with(new_size, || T::default());
  }

  pub fn reserve(&mut self, new_capacity: usize)
  {
    self.buf.reserve(new_capacity);
  }

  fn grow_auto(&mut self)
  {
    let single_layout = Layout::from_type::<T>();

    let old_capacity_bytes = self.buf.capacity * single_layout.size;
    assert!(old_capacity_bytes <= (core::usize::MAX / 4));

    let new_capacity = if self.buf.capacity == 0
    {
      1
    } else {
      self.buf.capacity * 2
    };

    self.reserve(new_capacity);
  }

  #[inline]
  pub fn len(&self) -> usize
  {
    self.size
  }

  #[inline]
  pub fn capacity(&self) -> usize
  {
    self.buf.capacity
  }

  pub fn push(&mut self, value: T)
  {
    if self.size == self.buf.capacity
    {
      self.grow_auto();
    }

    unsafe {
      self.buf.ptr.offset(self.size as isize).write(value);
    }

    self.size += 1;
  }

  pub fn pop(&mut self) -> Option<T>
  {
    if self.size == 0
    {
      None
    } else {
      let value = unsafe { self.buf.ptr.offset((self.size - 1) as isize).read() };

      self.size -= 1;
      Some(value)
    }
  }

  pub fn clear(&mut self)
  {
    if needs_drop::<T>()
    {
      unsafe {
        for i in 0..self.size
        {
          drop_in_place(self.buf.ptr.offset(i as isize));
        }
      }
    }

    self.size = 0;
  }

  #[inline]
  pub fn is_empty(&self) -> bool
  {
    self.size == 0
  }
}

impl<T, A> Array<T, Global>
{
  pub fn new() -> Self
  {
    Self::new_with(Global)
  }
}

impl<T, A: Allocator> Drop for Array<T, A>
{
  fn drop(&mut self)
  {
    if !self.buf.is_null() {
      self.clear();
    }
  }
}

impl<T, A: Allocator> Deref for Array<T, A>
{
  type Target = [T];

  #[inline]
  fn deref(&self) -> &Self::Target
  {
    unsafe { slice::from_raw_parts(self.buf.ptr, self.size) }
  }
}

impl<T, A: Allocator> DerefMut for Array<T, A>
{
  fn deref_mut(&mut self) -> &mut Self::Target
  {
    unsafe { slice::from_raw_parts_mut(self.buf.ptr, self.size) }
  }
}

impl<T, A: Allocator> Extend<T> for Array<T, A>
{
  fn extend<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = T>
  {
    for e in iter {
      self.push(e);
    }
  }
}

impl<'a, T: 'a, A: Allocator> Extend<&'a T> for Array<T, A>
where
    T: Clone,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        for e in iter
        {
            self.push(e.clone());
        }
    }
}


