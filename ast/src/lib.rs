mod builtin;
#[cfg(feature = "fold")]
mod fold_helpers;
pub mod generic {
    #![allow(clippy::derive_partial_eq_without_eq)]
    pub use crate::builtin::*;

    pub trait Custom<U> {
        fn custom(self) -> U;
    }

    include!("gen/generic.rs");
}
pub mod ranged;
// mod impls;  // TODO: temp disable
#[cfg(feature = "location")]
mod source_locator;
#[cfg(feature = "unparse")]
mod unparse;

pub use builtin::*;
pub use generic::*;
pub use rustpython_parser_core::{text_size, ConversionFlag};

pub type Suite<U = ()> = Vec<Stmt<U>>;

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

#[cfg(feature = "constant-optimization")]
mod optimizer;

#[cfg(feature = "constant-optimization")]
pub use optimizer::ConstantOptimizer;
