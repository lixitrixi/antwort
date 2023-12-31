pub mod clause;
pub mod error;
pub mod formula;
pub mod solver;

pub use clause::Clause;
pub use error::{Error, Result};
pub use formula::Formula;
pub use solver::{solve, Solution};

pub type Literal = i32;
