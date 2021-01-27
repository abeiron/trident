//! Implements the `Environment` that holds all of the entities.

use core::{
  any::TypeId,
  marker::PhantomData,
  ops::{Deref, DerefMut},
};

use crate::collections::HashMap;

mod data;
