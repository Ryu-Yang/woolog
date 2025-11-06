use pyo3::prelude::*;

#[pyfunction]
fn init(level: Option<&str>) -> PyResult<bool> {
    let level = level.unwrap_or("info");
    let level_filter = match level.to_lowercase().as_str() {
        "error" => tracing::LevelFilter::ERROR,
        "warn" => tracing::LevelFilter::WARN,
        "debug" => tracing::LevelFilter::DEBUG,
        "trace" => tracing::LevelFilter::TRACE,
        _ => tracing::LevelFilter::INFO,
    };

    Ok(crate::init_logger(level_filter).is_ok())
}

macro_rules! py_log {
    ($name:ident, $level:ident) => {
        #[pyfunction]
        fn $name(msg: &str) {
            tracing::$level!("{}", msg);
        }
    };
}

py_log!(trace, trace);
py_log!(debug, debug);
py_log!(info, info);
py_log!(warn, warn);
py_log!(error, error);

#[pymodule]
fn woolog(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_function(wrap_pyfunction!(trace, m)?)?;
    m.add_function(wrap_pyfunction!(debug, m)?)?;
    m.add_function(wrap_pyfunction!(info, m)?)?;
    m.add_function(wrap_pyfunction!(warn, m)?)?;
    m.add_function(wrap_pyfunction!(error, m)?)?;
    Ok(())
}