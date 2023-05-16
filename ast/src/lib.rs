mod builtin;
mod generic;
mod impls;
mod ranged;
#[cfg(feature = "unparse")]
mod unparse;

pub use builtin::*;
pub use generic::*;
pub use ranged::Ranged;
pub use rustpython_parser_core::{text_size, ConversionFlag};

pub trait Node {
    const NAME: &'static str;
    const FIELD_NAMES: &'static [&'static str];
}

#[cfg(feature = "fold")]
mod fold_helpers;
#[cfg(feature = "fold")]
pub mod fold {
    use super::generic::*;

    include!("gen/fold.rs");
}
#[cfg(feature = "fold")]
pub use fold::Fold;

#[cfg(feature = "visitor")]
mod visitor {
    use super::generic::*;

    include!("gen/visitor.rs");
}

#[cfg(feature = "location")]
pub mod located;
#[cfg(feature = "location")]
mod source_locator;
#[cfg(feature = "location")]
pub use rustpython_parser_core::source_code;

#[cfg(feature = "visitor")]
pub use visitor::Visitor;

#[cfg(feature = "constant-optimization")]
mod optimizer;
#[cfg(feature = "constant-optimization")]
pub use optimizer::ConstantOptimizer;

#[cfg(feature = "pyo3")]
pub mod pyo3;
#[cfg(feature = "pyo3-wrapper")]
pub mod pyo3_wrapper;
