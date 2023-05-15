use crate::Node;
use num_complex::Complex64;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList, PyTuple};

pub trait ToPyo3Ast {
    fn to_pyo3_ast(&self, py: Python) -> PyResult<Py<PyAny>>;
}

impl<T: ToPyo3Ast> ToPyo3Ast for Box<T> {
    #[inline]
    fn to_pyo3_ast(&self, py: Python) -> PyResult<Py<PyAny>> {
        (**self).to_pyo3_ast(py)
    }
}

impl<T: ToPyo3Ast> ToPyo3Ast for Option<T> {
    #[inline]
    fn to_pyo3_ast(&self, py: Python) -> PyResult<Py<PyAny>> {
        match self {
            Some(ast) => ast.to_pyo3_ast(py),
            None => Ok(py.None()),
        }
    }
}

impl<T: ToPyo3Ast> ToPyo3Ast for Vec<T> {
    fn to_pyo3_ast(&self, py: Python) -> PyResult<Py<PyAny>> {
        let list = PyList::empty(py);
        for item in self {
            let py_item = item.to_pyo3_ast(py)?;
            list.append(py_item)?;
        }
        Ok(list.into())
    }
}

impl ToPyo3Ast for crate::Identifier {
    #[inline]
    fn to_pyo3_ast(&self, py: Python) -> PyResult<Py<PyAny>> {
        Ok(self.as_str().to_object(py))
    }
}

impl ToPyo3Ast for crate::String {
    #[inline]
    fn to_pyo3_ast(&self, py: Python) -> PyResult<Py<PyAny>> {
        Ok(self.as_str().to_object(py))
    }
}

impl ToPyo3Ast for crate::Int {
    #[inline]
    fn to_pyo3_ast(&self, py: Python) -> PyResult<Py<PyAny>> {
        Ok((self.to_u32()).to_object(py))
    }
}

impl ToPyo3Ast for bool {
    #[inline]
    fn to_pyo3_ast(&self, py: Python) -> PyResult<Py<PyAny>> {
        Ok((*self as u32).to_object(py))
    }
}

impl ToPyo3Ast for crate::Constant {
    #[inline]
    fn to_pyo3_ast(&self, py: Python) -> PyResult<Py<PyAny>> {
        let value = match self {
            crate::Constant::None => py.None(),
            crate::Constant::Bool(bool) => bool.to_object(py),
            crate::Constant::Str(string) => string.to_object(py),
            crate::Constant::Bytes(bytes) => PyBytes::new(py, bytes).into(),
            crate::Constant::Int(int) => int.to_object(py),
            crate::Constant::Tuple(elts) => {
                let elts: PyResult<Vec<_>> = elts.iter().map(|c| c.to_pyo3_ast(py)).collect();
                PyTuple::new(py, elts?).into()
            }
            crate::Constant::Float(f64) => f64.to_object(py),
            crate::Constant::Complex { real, imag } => Complex64::new(*real, *imag).to_object(py),
            crate::Constant::Ellipsis => py.Ellipsis(),
        };
        Ok(value)
    }
}

// pub trait FromAst<A> {
//     fn from_ast(py: Python, ast: A) -> PyResult<Py<PyAny>>;
// }

pub trait ToPyo3Wrapper {
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>>;
}

impl<T: ToPyo3Wrapper> ToPyo3Wrapper for Box<T> {
    #[inline]
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        (**self).to_pyo3_wrapper(py)
    }
}

impl<T: ToPyo3Wrapper> ToPyo3Wrapper for Option<T> {
    #[inline]
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        match self {
            Some(ast) => ast.to_pyo3_wrapper(py),
            None => Ok(py.None()),
        }
    }
}

// for Vec needs refactoring
impl ToPyo3Wrapper for Vec<crate::ranged::Stmt> {
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        let list = PyList::empty(py);
        for item in self {
            let py_item = item.to_pyo3_wrapper(py)?;
            list.append(py_item)?;
        }
        Ok(list.into())
    }
}

impl ToPyo3Wrapper for Vec<crate::ranged::Expr> {
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        let list = PyList::empty(py);
        for item in self {
            let py_item = item.to_pyo3_wrapper(py)?;
            list.append(py_item)?;
        }
        Ok(list.into())
    }
}

impl ToPyo3Wrapper for Vec<crate::ranged::Arg> {
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        let list = PyList::empty(py);
        for item in self {
            let py_item = item.to_pyo3_wrapper(py)?;
            list.append(py_item)?;
        }
        Ok(list.into())
    }
}

impl ToPyo3Wrapper for Vec<crate::ranged::Pattern> {
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        let list = PyList::empty(py);
        for item in self {
            let py_item = item.to_pyo3_wrapper(py)?;
            list.append(py_item)?;
        }
        Ok(list.into())
    }
}

impl ToPyo3Wrapper for Vec<crate::Identifier> {
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        let list = PyList::empty(py);
        for item in self {
            let py_item = item.to_pyo3_wrapper(py)?;
            list.append(py_item)?;
        }
        Ok(list.into())
    }
}

impl ToPyo3Wrapper for Vec<crate::ranged::Keyword> {
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        let list = PyList::empty(py);
        for item in self {
            let py_item = item.to_pyo3_wrapper(py)?;
            list.append(py_item)?;
        }
        Ok(list.into())
    }
}

impl ToPyo3Wrapper for Vec<crate::ranged::Cmpop> {
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        let list = PyList::empty(py);
        for item in self {
            let py_item = item.to_pyo3_wrapper(py)?;
            list.append(py_item)?;
        }
        Ok(list.into())
    }
}

impl ToPyo3Wrapper for Vec<crate::ranged::Comprehension> {
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        let list = PyList::empty(py);
        for item in self {
            let py_item = item.to_pyo3_wrapper(py)?;
            list.append(py_item)?;
        }
        Ok(list.into())
    }
}

impl ToPyo3Wrapper for Vec<Option<crate::ranged::Expr>> {
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        let list = PyList::empty(py);
        for item in self {
            let py_item = item.to_pyo3_wrapper(py)?;
            list.append(py_item)?;
        }
        Ok(list.into())
    }
}

impl ToPyo3Wrapper for Vec<crate::ranged::Alias> {
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        let list = PyList::empty(py);
        for item in self {
            let py_item = item.to_pyo3_wrapper(py)?;
            list.append(py_item)?;
        }
        Ok(list.into())
    }
}

impl ToPyo3Wrapper for Vec<crate::ranged::Excepthandler> {
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        let list = PyList::empty(py);
        for item in self {
            let py_item = item.to_pyo3_wrapper(py)?;
            list.append(py_item)?;
        }
        Ok(list.into())
    }
}

impl ToPyo3Wrapper for Vec<crate::ranged::MatchCase> {
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        let list = PyList::empty(py);
        for item in self {
            let py_item = item.to_pyo3_wrapper(py)?;
            list.append(py_item)?;
        }
        Ok(list.into())
    }
}

impl ToPyo3Wrapper for Vec<crate::ranged::Withitem> {
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        let list = PyList::empty(py);
        for item in self {
            let py_item = item.to_pyo3_wrapper(py)?;
            list.append(py_item)?;
        }
        Ok(list.into())
    }
}

impl ToPyo3Wrapper for Vec<crate::ranged::TypeIgnore> {
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        let list = PyList::empty(py);
        for item in self {
            let py_item = item.to_pyo3_wrapper(py)?;
            list.append(py_item)?;
        }
        Ok(list.into())
    }
}

impl ToPyo3Wrapper for crate::Identifier {
    #[inline]
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        Ok(self.as_str().to_object(py))
    }
}

impl ToPyo3Wrapper for crate::String {
    #[inline]
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        Ok(self.as_str().to_object(py))
    }
}

impl ToPyo3Wrapper for crate::Int {
    #[inline]
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        Ok((self.to_u32()).to_object(py))
    }
}

impl ToPyo3Wrapper for bool {
    #[inline]
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        Ok((*self as u32).to_object(py))
    }
}

impl ToPyo3Wrapper for crate::Constant {
    fn to_pyo3_wrapper(&'static self, py: Python) -> PyResult<Py<PyAny>> {
        let value = match self {
            crate::Constant::None => py.None(),
            crate::Constant::Bool(bool) => bool.to_object(py),
            crate::Constant::Str(string) => string.to_object(py),
            crate::Constant::Bytes(bytes) => PyBytes::new(py, bytes).into(),
            crate::Constant::Int(int) => int.to_object(py),
            crate::Constant::Tuple(elts) => {
                let elts: PyResult<Vec<_>> = elts.iter().map(|c| c.to_pyo3_wrapper(py)).collect();
                PyTuple::new(py, elts?).into()
            }
            crate::Constant::Float(f64) => f64.to_object(py),
            crate::Constant::Complex { real, imag } => Complex64::new(*real, *imag).to_object(py),
            crate::Constant::Ellipsis => py.Ellipsis(),
        };
        Ok(value)
    }
}

include!("gen/to_pyo3_ranged.rs");
// include!("gen/to_pyo3_located.rs");

pub mod located {
    pub use super::AST;
    use super::*;
    // include!("gen/pyo3_located.rs");
}

pub mod ranged {
    pub use super::AST;
    use super::*;
    include!("gen/pyo3_ranged.rs");
}

#[pyclass(module = "rustpython_ast", subclass)]
pub struct AST;

#[pymethods]
impl AST {
    #[new]
    fn new() -> Self {
        Self
    }
}

/// A Python module implemented in Rust.
fn init_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AST>()?;

    let ast = m.getattr("AST")?;
    let fields = PyTuple::empty(py);
    ast.setattr("_fields", fields)?;

    Ok(())
}
