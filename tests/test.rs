#[cfg(test)]
mod tests {
    // use super::*;
    use woolog;

    #[test]
    fn test_logger_init_and_log() {
        // 初始化日志（测试时建议使用 TRACE 以捕获所有）
        let _ = woolog::init_logger(tracing::Level::TRACE.into());

        // 调用各个日志函数（应不会 panic）
        woolog::trace("Test trace message");
        woolog::debug("Test debug message");
        woolog::info("Test info message");
        woolog::warn("Test warn message");
        woolog::error("Test error message");

        // 如果初始化两次，应成功（因为有 OnceCell 保护）
        let _ = woolog::init_logger(tracing::Level::INFO.into());
    }

    #[test]
    fn test_logger_with_null_ffi() {
        // 测试 FFI 函数传入 null 是否安全（虽然这是 FFI，但可从 Rust 调用）
        use std::ptr;
        assert!(woolog::woolog_init(ptr::null()));
        woolog::woolog_info(b"Hello from FFI test\0".as_ptr() as *const i8);
    }
}