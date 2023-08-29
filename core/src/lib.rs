#![doc(html_logo_url = "https://raw.githubusercontent.com/RustPython/RustPython/main/logo.png")]
#![doc(html_root_url = "https://docs.rs/rustpython-parser-core/")]

mod error;
mod format;
pub mod mode;
#[cfg(feature = "location")]
pub mod source_code;

pub use error::BaseError;
pub use format::ConversionFlag;
pub use mode::Mode;

#[cfg(feature = "location")]
pub use rustpython_parser_vendored::source_location;
pub use rustpython_parser_vendored::text_size;
