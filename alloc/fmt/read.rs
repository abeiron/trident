use super::Error;
use super::Result;

pub trait Read 
{
  fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

  fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> 
  {
            while !buf.is_empty()
        {
            match self.read(buf)
            {
                Ok(0) => break,
                Ok(n) => buf = &mut buf[n..],
                Err(e) => return Err(e),
            }
        }

        if !buf.is_empty()
        {
            Err(Error::UnexpectedEof)
        }
        else
        {
            Ok(())
        }
      
  }
}