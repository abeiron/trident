#[macro_export]
pub macro bitflags {
(
  $vis:vis enum $name:ident: $type:ty
  {
    $(
      $vname:ident = $value:expr,
    )+
  }
  ) => {
  #[repr(transparent)]
  #[derive(Clone, Copy, PartialEq, Hash, PartialOrd)]
  $vis struct $name
  {
    bits: $type,
  }

  impl $name
  {
    $(
      pub const $vname: Self = Self{ bits: $value };
    )+

    #[inline]
    pub fn none() -> Self
    {
      Self {
        bits: 0,
      }
    }

    #[inline]
    pub fn all() -> Self
    {
      Self {
        bits: $($value)|+,
      }
    }

    #[inline]
    pub fn as_raw(&self) -> $type
    {
      self.bits
    }

    #[inline]
    pub fn contains(&self, other: Self) -> bool
    {
      (self.bits & other.bits) == other.bits
    }

    #[inline]
    pub fn insert(&mut self, other: Self)
    {
      self.bits |= other.bits;
    }

    #[inline]
    pub fn toggle(&mut self, other: Self)
    {
      self.bits ^= other.bits;
    }

    #[inline]
    pub fn remove(&mut self, other: Self)
    {
      self.bits &= !other.bits;
    }

    #[inline]
    pub fn is_empty(&self) -> bool
    {
      self.bits == 0
    }
  }

  impl Default for $name
  {
    fn default() -> $name
    {
      $name { bits: 0 }
    }
  }

  impl core::convert::TryFrom<$type> for $name
  {
    type Error = ();

    #[inline]
    fn try_from(bits: $type) -> Result<Self, ()>
    {
      if (bits & Self::all().bits) != bits
      {
        Err(())
      }
      else
      {
        Ok(Self {
          bits
        })
      }
    }
  }

  impl core::fmt::Debug for $name
  {
    #[allow(unused_assignments)]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error>
    {
      let mut first = true;
      f.write_str(stringify!($name))?;
      f.write_str("(")?;

      $(
        if self.contains(Self::$vname)
        {
          if !first
          {
            f.write_str(" | ")?;
          }

          f.write_str(stringify!($vname))?;
          first = false;
        }
      )+

      f.write_str(")")?;
      Ok(())
    }
  }

  impl core::ops::BitOr for $name
  {
    type Output = Self;

    #[inline]
    fn bitor(mut self, other: Self) -> Self
    {
      self.insert(other);
      return self;
    }
  }

  impl core::ops::BitOrAssign for $name
  {
    #[inline]
    fn bitor_assign(&mut self, other: Self)
    {
      self.insert(other);
    }
  }

  impl core::ops::Sub for $name
  {
    type Output = Self;

    #[inline]
    fn sub(mut self, other: Self) -> Self
    {
      self.remove(other);
      return self;
    }
  }

  impl core::ops::SubAssign for $name
  {
    #[inline]
    fn sub_assign(&mut self, other: Self)
    {
      self.remove(other);
    }
  }

  impl core::ops::BitAnd for $name
  {
    type Output = Self;

    #[inline]
    fn bitand(mut self, other: Self) -> Self
    {
      self.bits &= other.bits;
      return self;
    }
  }

  impl core::ops::BitAndAssign for $name
  {
    #[inline]
    fn bitand_assign(&mut self, other: Self)
    {
      self.bits &= other.bits;
    }
  }

  impl core::ops::BitXor for $name
  {
    type Output = Self;

    #[inline]
    fn bitxor(mut self, other: Self) -> Self
    {
      self.toggle(other);
      return self;
    }
  }

  impl core::ops::BitXorAssign for $name
  {
    #[inline]
    fn bitxor_assign(&mut self, other: Self)
    {
      self.toggle(other);
    }
  }
}
}