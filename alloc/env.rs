//! Implements the `Environment` that holds all of the entities.

use core::ops::{Deref, DerefMut};

use crate::{
  any::{Any, TypeId},
  cell::{Ref, RefMut, TrustCell},
  collections::HashMap,
  marker::PhantomData,
};
use crate::ptr::unique::Unq;

mod data;
mod entry;
#[macro_use]
mod setup;


/// Allows to fetch a resource in a system immutably.
///
/// If the resource isn't strictly required, you should use `Option<Fetch<T>>`.
///
/// # Type Parameters
///
/// * `T`: The type of the resource.
pub struct Fetch<'a, T: 'a> 
{
  inner: Ref<'a, dyn Resource>,
  phantom: PhantomData<&'a T>,
}

impl<'a, T> Deref for Fetch<'a, T>
where
    T: Resource,
{
  type Target = T;
 
  #[inline]
  fn deref(&self) -> &Self::Target
  {
    unsafe { self.inner.downcast_ref_unchecked() }
  }
}

impl<'a, T> Clone for Fetch<'a, T>
{
  fn clone(&self) -> Self
  {
    Fetch {
      inner: self.inner.clone(),
      phantom: PhantomData,
    }
  }
}

/// Allows to fetch a resource in a system mutably.
///
/// If the resource isn't strictly required, you should use
/// `Option<FetchMut<T>>`.
///
/// # Type Parameters
/// 
/// * `T`: The type of the resource.
pub struct FetchMut<'a, T: 'a>
{
  inner: RefMut<'a, dyn Resource>,
  phantom: PhantomData<&'a T>,
}

impl<'a, T> Deref for FetchMut<'a, T>
where
    T: Resource,
{
  type Target = T;
  
  fn deref(&self) -> &Self::Target
  {
    unsafe { self.inner.downcast_ref_unchecked() }
  }
}

impl<'a, T> DerefMut for FetchMut<'a, T>
where
    T: Resource,
{
  fn deref_mut(&mut self) ->  &mut Self::Target
  {
    unsafe { self.inner.downcast_mut_unchecked() }
  }
}

/// A resource is a data slot which lives in the `Environment`
/// and can only be accessed according to Rust's typical borrowing model.
/// (i.e. one writer or multiple readers).
pub trait Resource: Any + Send + Sync + 'static {}

impl<T> Resource for T where T: Any + Send + Sync {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResourceId
{
  type_id: TypeId,
  dynamic_id: u64,
}

impl ResourceId 
{
  /// Creates a new resource id from a given type.
  #[inline]
  pub fn new<T: Resource>() -> Self
  {
    ResourceId::new_with_dynamic_id::<T>(0)
  }
  
  /// Creates a new resource from a raw type ID.
  #[inline]
  pub fn from_type_id(type_id: TypeId) -> Self
  {
    ResourceId::from_type_id_and_dynamic_id(type_id, 0)
  }
  
  /// Creates a new resource id from a given type and a `dynamic_id`.
  ///
  /// This is usually not what you want (unless you're implementing 
  /// scripting to define resources at run-time).
  ///
  /// Creating resource IDs with a `dynamic_id` unequal to `0` is only
  /// recommended for special types that are specifically designed for
  /// scripting; most of the time, we just assume that resources are
  /// identified only by their type.
  #[inline]
  pub fn new_with_dynamic_id<T: Resource>(dynamic_id: u64) -> Self
  {
    ResourceId::from_type_id_and_dynamic_id(TypeId::of::<T>(), dynamic_id)
  }
  
  /// Create a new resource id from a raw type id and a "dynamic id" (see type
  /// documentation).
  #[inline]
  pub fn from_type_id_and_dynamic_id(type_id: TypeId, dynamic_id: u64) -> Self
  {
    ResourceId {
      type_id,
      dynamic_id,
    }
  }
  
  fn assert_same_type_id<R: Resource>(&self) 
  {
    let res_id0 = ResourceId::new::<R>();
    assert_eq!(
      res_id0.type_id, self.type_id,
      "Passed a `ResoureId` with a wrong type id"
    );
  }
}

/// A [`Resource`] container, which provides methods to insert, access,
/// and manage the contained resources.
/// 
/// Many methods take `&self` which works because everything is stored
/// with **interior mutability**. In the event that you violate Rust's
/// rules for borrowing, you'll get a panic.
///
/// # Resource Ids
/// 
/// Resources are identified by `ResourceId`s, which consist of a
/// `TypeId`.
#[derive(Default)]
pub struct Environment
{
  resources: HashMap<ResourceId, TrustCell<Unq<dyn Resource>>>,
}

impl Environment
{
  /// Creates a new, empty resource container.
  ///
  /// # Usage
  ///
  /// ```no_compile
  /// ...
  /// let e = Environment::empty();
  /// ...
  /// ```
  pub fn empty() -> Self
  {
    Default::default()
  }
}
