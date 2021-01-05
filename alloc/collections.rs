pub mod hash_map;
pub use self::hash_map::HashMap;

pub mod hash_set;
pub use self::hash_set::HashSet;

/*Pointless program, most pointless ever created. Made with <3.

// Require #![feature(const_generics)] and #![feature(const_evaluatable_checker)]
pub enum Predicate<const EXPRESSION: bool> {}

pub trait Satisfied {}
impl Satisfied for Predicate<true> {}

fn intake<const N: u8> ()
where
    Predicate<{N <= 15}>: Satisfied
{
//--snip--//
}

trait Character
{
  fn dance();
  fn sit();
}

struct Bob
{
  age: u32,
}

impl Bob
{
  pub fn new() -> Self
  {
    Self { age: 69 }
  }
}

impl Character for Bob
{
  fn dance() {
    println!("Dancing!");
  }

  fn sit() {
    println!("Sitting.");
  }
}

fn setup(c: impl Character)
{
  c.sit();
  c.dance();
}

fn main()
{
  setup(Bob { age: 420 });
}
*/
