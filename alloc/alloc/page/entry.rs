
pub struct Entry
{
  pub entry: i64,
}

impl Entry
{
  pub fn valid(&self) -> bool
  {
    self.get() & EntryBits::Valid.val() != 0
  }

  pub fn set(&mut self, entry: i64)
  {
    self.entry = entry;
  }

  pub fn get(&self) -> i64
  {
    self.entry
  }
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum EntryBits
{
  None = 0,
  Valid = 1 << 0,
  Read = 1 << 1,
  Write = 1 << 2,
  Execute = 1 << 3,
  User = 1 << 4,
  Global = 1 << 5,
  Access = 1 << 6,
  Dirty = 1 << 7,

  // Convenience combinations.
  ReadWrite = 1 << 1 | 1 << 2,
  ReadExecute = 1 << 1 | 1 << 3,
  ReadWriteExecute = 1 << 1 | 1 << 2 | 1 << 3,

  // User convenience combinations.
  UserReadWrite = 1 << 1 | 1 << 2 | 1 << 4,
  UserReadExecute = 1 << 1 | 1 << 3 | 1 << 4,
  UserReadWriteExecute = 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4,
}

impl EntryBits
{
  pub fn val(self) -> usize
  {
    self as usize
  }
}
