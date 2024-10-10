use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct PieceNotFoundError;

impl Display for PieceNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("no piece found at the specified position")
    }
}