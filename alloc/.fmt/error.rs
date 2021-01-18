
/// Input/Output error type.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Error
{
  InvalidOpenOptions,
  CannotOpen,
  CannotSeek,
  CannotRead,
  CannotWrite,
  CannotFlush,
  BufferTooLarge,
  UnexpectedEof,
}