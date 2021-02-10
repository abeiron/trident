//! A simple console host.

pub struct ConHost 
{
  x: u32,
  y: u32,
  buf: *mut u8,
}

impl ConHost 
{
  pub fn init() -> ConHost 
  {
    ConHost {
      x: 0,
      y: 0,
      buf: () as *mut u8,
    }
  }
}
