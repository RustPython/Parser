use crate::text_size::{TextRange, TextSize};

pub trait Ranged {
    fn range(&self) -> TextRange;

    fn start(&self) -> TextSize {
        self.range().start()
    }

    fn end(&self) -> TextSize {
        self.range().end()
    }
}

pub use crate::builtin::*;
include!("gen/ranged.rs");
