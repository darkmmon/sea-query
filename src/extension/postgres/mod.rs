pub use expr::*;
pub use extension::*;
pub use func::*;
pub use interval::*;
pub use types::*;

use crate::types::BinOper;

pub(crate) mod expr;
pub(crate) mod extension;
pub(crate) mod func;
pub(crate) mod interval;
pub(crate) mod types;

/// Binary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgBinOper {
    ILike,
    NotILike,
    Matches,
    Contains,
    Contained,
    Concatenate,
    Similarity,
    WordSimilarity,
    StrictWordSimilarity,
    SimilarityDistance,
    WordSimilarityDistance,
    StrictWordSimilarityDistance,
    /// `->`. Retrieves JSON field as JSON value.
    GetJsonField,
    /// `->>`. Retrieves JSON field and casts it to an appropriate SQL type.
    CastJsonField,
    /// `~` Regex operator.
    Regex,
    /// `~*`. Regex operator with case insensitive matching.
    RegexCaseInsensitive,
}

impl From<PgBinOper> for BinOper {
    fn from(o: PgBinOper) -> Self {
        Self::PgOperator(o)
    }
}
