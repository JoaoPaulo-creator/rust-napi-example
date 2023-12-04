#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn fibonacci_rust(n: u32) -> u32 {
  match n {
    1 | 2 => 1,
    _ => fibonacci_rust(n - 1) + fibonacci_rust(n - 2),
  }
}
