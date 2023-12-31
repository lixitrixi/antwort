#[derive(Debug, PartialEq)]
pub enum Error {
    Unsatisfiable,
    InvalidLiteral,
}

pub type Result<T> = std::result::Result<T, Error>;
