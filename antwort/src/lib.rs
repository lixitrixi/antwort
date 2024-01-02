// Re-export core library
pub use antwort_core::ast::*;
pub use antwort_core::*;

pub use antwort_solver as solver;

pub use antwort_macros as macros;

pub mod rule_engine;
pub use rule_engine::_RULES_DISTRIBUTED_SLICE;
