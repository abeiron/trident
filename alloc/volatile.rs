//! TODO: Document module.

use core::ops::{Deref, DerefMut};

pub struct Volatile<T>
where
    T: Deref + DerefMut + Sized,
{

}
