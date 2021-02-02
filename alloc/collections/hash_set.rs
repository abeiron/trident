use core::{borrow::Borrow, hash::*};

use crate::{
  alloc::{AllocRef, Global},
  collections::HashMap,
};

pub struct HashSet<T, A = Global>
  where
      T: Sized + Eq + Hash,
      A: AllocRef + Clone,
{
  map: HashMap<T, (), A>,
}

impl<T, A> HashSet<T, A>
  where
      T: Sized + Eq + Hash,
      A: AllocRef + Clone,
{
  pub fn new_with(alloc: A) -> Self
  {
    Self {
      map: HashMap::new_with(alloc),
    }
  }

  pub fn contains<Q>(&self, val: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
  {
    self.map.contains(val)
  }

  pub fn remove<Q>(&mut self, val: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
  {
    self.map.remove(val)
  }

  pub fn insert(&mut self, val: T) -> bool
  {
    self.map.insert(val, ())
  }

  pub fn len(&self) -> usize
  {
    self.map.len()
  }
}

impl<T> HashSet<T, Global>
  where
      T: Sized + Eq + Hash,
{
  pub fn new() -> Self
  {
    Self::new_with(Global)
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn contains()
  {
    let mut set = HashSet::new();
    assert!(!set.contains(&5));
    assert!(set.insert(5));
    assert!(set.insert(4));
    assert!(set.insert(6));
    assert!(set.contains(&5));
    assert!(set.contains(&4));
    assert!(set.contains(&6));
    assert!(!set.contains(&0));
    assert!(!set.contains(&9000));
    assert!(!set.contains(&51));
  }

  #[test]
  fn insert()
  {
    let mut set = HashSet::new();

    assert!(set.insert(5));
    assert!(set.insert(4));
    assert!(set.insert(6));
    assert!(!set.insert(5));
    assert!(!set.insert(6));
    assert!(set.insert(7));
  }

  #[test]
  fn len()
  {
    let mut set = HashSet::new();

    assert!(set.len() == 0);
    set.insert(3);
    assert!(set.len() == 1);
    set.insert(1);
    assert!(set.len() == 2);
    set.insert(3);
    assert!(set.len() == 2);
  }

  #[test]
  fn remove()
  {
    let mut set = HashSet::new();

    set.insert(1);
    set.insert(2);
    set.insert(3);

    assert!(!set.remove(&4));
    assert!(set.len() == 3);
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));

    assert!(set.remove(&2));
    assert!(set.len() == 2);
    assert!(set.contains(&1));
    assert!(!set.contains(&2));
    assert!(set.contains(&3));

    assert!(!set.remove(&2));
    assert!(set.len() == 2);
    assert!(set.contains(&1));
    assert!(!set.contains(&2));
    assert!(set.contains(&3));

    assert!(set.remove(&3));
    assert!(set.len() == 1);
    assert!(set.contains(&1));
    assert!(!set.contains(&2));
    assert!(!set.contains(&3));
  }

  #[test]
  fn zst()
  {
    let mut set = HashSet::new();

    assert!(!set.contains(&()));
    assert!(!set.remove(&()));
    assert!(set.len() == 0);
    assert!(set.insert(()));
    assert!(!set.insert(()));
    assert!(set.contains(&()));
    assert!(set.len() == 1);
    assert!(set.remove(&()));
    assert!(!set.contains(&()));
    assert!(!set.remove(&()));
    assert!(set.len() == 0);
  }
}
