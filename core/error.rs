//! Defines an error handling trait and some basic implementations.

use core::fmt::{Debug, Display};

pub trait Error: Debug + Display
{

}
