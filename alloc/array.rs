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
pub use self::small_array::SmallArray;

use crate::alloc::{AllocRef, Global, Layout};

pub struct Array<T, A: AllocRef = Global>
{
  size: usize,
  buf: RawArray<T, A>,
}

impl<T, A: AllocRef> Array<T, A>
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

impl<T> Array<T, Global>
{
  pub fn new() -> Self
  {
    Self::new_with(Global)
  }
}

impl<T, A: AllocRef> Drop for Array<T, A>
{
  fn drop(&mut self)
  {
    if !self.buf.ptr.is_null() {
      self.clear();
    }
  }
}

impl<T, A: AllocRef> Deref for Array<T, A>
{
  type Target = [T];

  #[inline]
  fn deref(&self) -> &Self::Target
  {
    unsafe { slice::from_raw_parts(self.buf.ptr, self.size) }
  }
}

impl<T, A: AllocRef> DerefMut for Array<T, A>
{
  fn deref_mut(&mut self) -> &mut Self::Target
  {
    unsafe { slice::from_raw_parts_mut(self.buf.ptr, self.size) }
  }
}

impl<T, A: AllocRef> Extend<T> for Array<T, A>
{
  fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item=T>
  {
    for e in iter {
      self.push(e);
    }
  }
}

impl<'a, T: 'a, A: AllocRef> Extend<&'a T> for Array<T, A>
  where
      T: Clone,
{
  fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item=&'a T>,
  {
    for e in iter
    {
      self.push(e.clone());
    }
  }
}

impl<T, A: AllocRef> FromIterator<T> for Array<T, A>
  where
      A: Default,
{
  fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item=T>,
  {
    let mut array = Array::new_with(A::default());
    array.extend(iter);
    return array;
  }
}

pub struct IntoIter<T, A: AllocRef>
{
  inner: Array<T, A>,
  current: usize,
  size: usize,
}

impl<T, A: AllocRef> Iterator for IntoIter<T, A>
{
  type Item = T;

  fn next(&mut self) -> Option<T>
  {
    if self.current >= self.size
    {
      None
    } else {
      unsafe {
        let index = self.current;
        self.current += 1;
        Some(read(self.inner.buf.ptr.offset(index as isize)))
      }
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>)
  {
    let remaining = self.size - self.current;
    (remaining, Some(remaining))
  }
}

impl<T, A: AllocRef> Drop for IntoIter<T, A>
{
  fn drop(&mut self)
  {
    // Drop the remaining elements if we didn't iter
    // until the end.
    if needs_drop::<T>()
    {
      unsafe {
        for i in self.current..self.size
        {
          drop_in_place(self.inner.buf.ptr.offset(i as isize))
        }
      }
    }

    // Set size of the array to 0 so it doesn't drop anything else.
    self.inner.size = 0;
  }
}

#[cfg(test)]
mod tests
{
  use super::*;
  use core::cell::Cell;

  struct DropCheck<'a>
  {
    pub dropped: &'a Cell<i32>,
  }

  impl<'a> DropCheck<'a>
  {
    fn new(b: &'a Cell<i32>) -> Self
    {
      Self { dropped: b }
    }
  }

  impl<'a> Drop for DropCheck<'a>
  {
    fn drop(&mut self)
    {
      self.dropped.set(self.dropped.get() + 1);
    }
  }

  #[test]
  fn push_pop()
  {
    let mut a = Array::new();

    a.push(1);
    a.push(2);
    a.push(3);
    a.push(4);
    a.push(5);

    assert!(a.len() == 5);
    assert!(a.pop() == Some(5));
    assert!(a.pop() == Some(4));
    assert!(a.pop() == Some(3));
    assert!(a.len() == 2);

    a.push(3);
    a.push(4);
    a.push(5);

    assert!(a.len() == 5);
    assert!(a.pop() == Some(5));
    assert!(a.pop() == Some(4));
    assert!(a.pop() == Some(3));
    assert!(a.pop() == Some(2));
    assert!(a.pop() == Some(1));
    assert!(a.pop() == None);
    assert!(a.pop() == None);
    assert!(a.pop() == None);
    assert!(a.pop() == None);
    assert!(a.len() == 0);
  }

  #[test]
  fn drop()
  {
    let dropped = Cell::new(0);

    {
      let mut a = Array::new();
      a.push(DropCheck::new(&dropped));
    }

    assert!(dropped.get() == 1);
  }

  fn sum_slice(slice: &[i32]) -> i32
  {
    slice.iter().sum()
  }

  fn double_slice(slice: &mut [i32])
  {
    slice.iter_mut().for_each(|x| *x = *x * 2);
  }

  #[test]
  fn slice()
  {
    let mut a = Array::new();

    a.push(1);
    a.push(2);
    a.push(3);
    a.push(4);
    a.push(5);

    assert!(sum_slice(&a) == 15);
    double_slice(&mut a);

    assert!(a[0] == 2);
    assert!(a[1] == 4);
    assert!(a[2] == 6);
    assert!(a[3] == 8);
    assert!(a[4] == 10);
  }

  #[test]
  fn subslice()
  {
    let mut a = Array::new();

    a.push(1);
    a.push(2);
    a.push(3);
    a.push(4);
    a.push(5);

    assert!(sum_slice(&a[1..3]) == 5);
    assert!(sum_slice(&a[1..=3]) == 9);

    double_slice(&mut a[1..3]);

    assert!(a[0] == 1);
    assert!(a[1] == 4);
    assert!(a[2] == 6);
    assert!(a[3] == 4);
    assert!(a[4] == 5);

    double_slice(&mut a[1..=3]);

    assert!(a[0] == 1);
    assert!(a[1] == 8);
    assert!(a[2] == 12);
    assert!(a[3] == 8);
    assert!(a[4] == 5);
  }

  #[test]
  fn iter()
  {
    let mut a = Array::new();

    a.push(1);
    a.push(2);
    a.push(3);
    a.push(4);
    a.push(5);

    for (i, value) in a.iter().enumerate()
    {
      assert!(i as i32 == value - 1);
    }

    for (i, value) in a.iter_mut().enumerate()
    {
      assert!(i as i32 == *value - 1);
    }
  }

  #[test]
  fn resize()
  {
    let mut a = Array::new();

    a.push(1);
    a.resize(3, 7);
    a.push(2);
    a.push(3);

    assert!(a[0] == 1);
    assert!(a[1] == 7);
    assert!(a[2] == 7);
    assert!(a[3] == 2);
    assert!(a[4] == 3);
  }

  #[test]
  fn extend()
  {
    let mut a = Array::new();

    a.push(1);
    a.extend([7, 8].iter());
    a.extend(&[2, 3]);

    assert!(a[0] == 1);
    assert!(a[1] == 7);
    assert!(a[2] == 8);
    assert!(a[3] == 2);
    assert!(a[4] == 3);
  }

  #[test]
  fn zst()
  {
    let mut a = Array::new();

    a.push(());
    a.push(());
    a.push(());
    assert!(a.len() == 3);

    assert!(a[1] == ());

    a.clear();
    assert!(a.len() == 0);
  }

  #[test]
  fn into_iter_drain_all()
  {
    let dropped1 = Cell::new(0);
    let dropped2 = Cell::new(0);
    let dropped3 = Cell::new(0);

    {
      let mut a = Array::new();
      a.push(DropCheck::new(&dropped1));
      a.push(DropCheck::new(&dropped2));
      a.push(DropCheck::new(&dropped3));

      let mut i = a.into_iter();
      assert!(i.next().is_some());
      assert!(i.next().is_some());
      assert!(i.next().is_some());
      assert!(i.next().is_none());
    }

    assert!(dropped1.get() == 1);
    assert!(dropped2.get() == 1);
    assert!(dropped3.get() == 1);
  }

  #[test]
  fn into_iter_drain_some()
  {
    let dropped1 = Cell::new(0);
    let dropped2 = Cell::new(0);
    let dropped3 = Cell::new(0);

    {
      let mut a = Array::new();
      a.push(DropCheck::new(&dropped1));
      a.push(DropCheck::new(&dropped2));
      a.push(DropCheck::new(&dropped3));

      let mut i = a.into_iter();
      assert!(i.next().is_some());
      assert!(i.next().is_some());
    }

    assert!(dropped1.get() == 1);
    assert!(dropped2.get() == 1);
    assert!(dropped3.get() == 1);
  }

  #[test]
  fn into_iter_drain_none()
  {
    let dropped1 = Cell::new(0);
    let dropped2 = Cell::new(0);
    let dropped3 = Cell::new(0);

    {
      let mut a = Array::new();
      a.push(DropCheck::new(&dropped1));
      a.push(DropCheck::new(&dropped2));
      a.push(DropCheck::new(&dropped3));

      a.into_iter();
    }

    assert!(dropped1.get() == 1);
    assert!(dropped2.get() == 1);
    assert!(dropped3.get() == 1);
  }
}
