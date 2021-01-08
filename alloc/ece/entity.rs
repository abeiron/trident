//! Implements the `Entity` and `EntityId` types for an Entity-Component-Engine submodule.

use core::hash::Hasher;
use crate::hash::SipHash;

// -- Entity-related code -- //

pub const MAX_ENTITIES: usize = usize::MAX;

pub struct HashedId
{
  inner: EntityId,
}

impl HashedId
{
  pub fn new(i: u64) -> Self
  {
    let mut hasher = SipHash::default();
    hasher.write_u64(i);

    Self { inner: hasher.finish() }
  }
}

pub type EntityId = u64;

pub trait Entity
{
  type Target = HashedId;
}
