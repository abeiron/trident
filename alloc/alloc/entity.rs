//! Implements the `Entity` and `EntityId` types for an Entity-Component-Engine submodule.

use core::hash::Hasher;
use crate::hash::SipHash;

// -- Entity-related code -- //

pub const MAX_ENTITIES: usize = usize::MAX;

pub struct HashedIndex
{
  inner: Index,
}

impl HashedIndex
{
  pub fn new(i: u64) -> Self
  {
    let mut hasher = SipHash::default();
    hasher.write_u64(i);

    Self { inner: hasher.finish() }
  }
}

pub type Index = u64;

pub struct Entity(HashedIndex);

impl Entity
{
  #[cfg(test)]
  pub fn new(index: Index) -> Self
  {
    let hi = HashedIndex::new(index);
    
    Self(hi)
  }
  
  /// Returns the hashed index of the `Entity`.
  #[inline]
  pub fn id(self) -> HashedIndex
  {
    self.0
  }
}
