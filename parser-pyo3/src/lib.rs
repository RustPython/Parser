use pyo3::prelude::*;

use rustpython_ast::{
    pyo3::{ToPyo3Ast, ToPyo3Wrapper},
    Fold,
};

#[pyfunction]
fn parse_wrap(a: &str, py: Python) -> PyResult<PyObject> {
    let parsed = rustpython_parser::parse(a, rustpython_parser::Mode::Module, "<unknown>")
        .map_err(|e| PyErr::new::<pyo3::exceptions::PySyntaxError, _>(e.to_string()))?;
    // let parsed = rustpython_ast::source_code::SourceLocator::new(a).fold(parsed).unwrap();
    let parsed = parsed.module().unwrap();
    let ast_module = PyModule::import(py, "_ast")?;

    let parsed = Box::leak(Box::new(parsed));
    parsed.to_pyo3_wrapper(py)
}

#[pyfunction]
fn parse(a: &str, py: Python) -> PyResult<PyObject> {
    use rustpython_parser::{ast::fold::Fold, source_code::SourceLocator};
    let parsed = rustpython_parser::parse(a, rustpython_parser::Mode::Module, "<unknown>")
        .map_err(|e| PyErr::new::<pyo3::exceptions::PySyntaxError, _>(e.to_string()))?
        // .module().unwrap()
        ;
    // let located = SourceLocator::new(a).fold(parsed).unwrap();
    let x = parsed.module().unwrap();
    x.to_pyo3_ast(py)
}

#[pymodule]
fn rustpython_parser_pyo3(py: Python, m: &PyModule) -> PyResult<()> {
    // let ast = PyModule::new(py, "ast")?;
    // rustpython_ast::pyo3::located::add_to_module(py, ast)?;
    // m.add_submodule(ast)?;

    let ast = PyModule::new(py, "unlocated_ast")?;
    rustpython_ast::pyo3::ranged::add_to_module(py, ast)?;
    m.add_submodule(ast)?;

    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(parse_wrap, m)?)?;

    Ok(())
}
