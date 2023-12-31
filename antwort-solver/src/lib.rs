mod clause;
mod error;
mod formula;
mod solver;

pub use clause::Clause;
pub use error::{Error, Result};
pub use formula::Formula;
pub use solver::{solve, Solution};

pub type Literal = i32;
