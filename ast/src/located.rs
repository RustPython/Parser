#![allow(clippy::derive_partial_eq_without_eq)]
use crate::source_code::{SourceLocation, SourceRange};

pub trait Located {
    fn range(&self) -> SourceRange;

    fn location(&self) -> SourceLocation {
        self.range().start
    }

    fn end_location(&self) -> Option<SourceLocation> {
        self.range().end
    }
}

pub use crate::builtin::*;
include!("gen/located.rs");
