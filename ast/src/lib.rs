mod builtin;
#[cfg(feature = "fold")]
mod fold_helpers;
mod generic {
    #![allow(clippy::derive_partial_eq_without_eq)]
    pub use crate::builtin::*;

    include!("gen/generic.rs");
}
mod impls;
mod ranged;
#[cfg(feature = "location")]
mod source_locator;
#[cfg(feature = "unparse")]
mod unparse;

pub use builtin::*;
pub use generic::*;
pub use rustpython_parser_core::{text_size, ConversionFlag};
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;

pub type Suite<R = TextRange> = Vec<Stmt<R>>;

#[cfg(feature = "fold")]
pub mod fold {
    use super::generic::*;

    include!("gen/fold.rs");
}

#[cfg(feature = "visitor")]
mod visitor {
    use super::generic::*;

    include!("gen/visitor.rs");
}

#[cfg(feature = "location")]
pub mod located;

#[cfg(feature = "location")]
pub use rustpython_parser_core::source_code;
#[cfg(feature = "visitor")]
pub use visitor::Visitor;

pub trait Ranged {
    fn range(&self) -> TextRange;

    fn start(&self) -> TextSize {
        self.range().start()
    }

    fn end(&self) -> TextSize {
        self.range().end()
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct EmptyRange<R> {
    phantom: PhantomData<R>,
}

impl<R> Display for EmptyRange<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("()")
    }
}

impl<R> Debug for EmptyRange<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<R> Default for EmptyRange<R> {
    fn default() -> Self {
        EmptyRange {
            phantom: PhantomData,
        }
    }
}

#[cfg(feature = "constant-optimization")]
mod optimizer;

#[cfg(feature = "constant-optimization")]
pub use optimizer::ConstantOptimizer;
use rustpython_parser_core::text_size::{TextRange, TextSize};
