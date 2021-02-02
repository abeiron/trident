use super::entry::Entry;

pub struct Table
{
  pub entries: [Entry; 512]
}

impl Table
{
  pub fn len() -> usize
  {
    512
  }
}
