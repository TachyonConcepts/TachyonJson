#![allow(unsafe_op_in_unsafe_fn)]
pub mod buffer;
pub mod value;
pub use value::{TachyonPair, TachyonValue, TachyonObject, FastStr};
pub use buffer::TachyonBuffer;
#[macro_use]
pub mod macros;