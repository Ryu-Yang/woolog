use once_cell::sync::OnceCell;
use std::ffi::{CStr};
use std::os::raw::c_char;
use tracing::{event};
use tracing_subscriber::{
    filter::LevelFilter,
    layer::SubscriberExt,
    EnvFilter, Registry,
};

static LOGGER_INITIALIZED: OnceCell<()> = OnceCell::new();

/// 初始化日志系统
pub fn init_logger(level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    if LOGGER_INITIALIZED.get().is_some() {
        return Ok(());
    }

    // 创建格式化层
    let format_layer = tracing_subscriber::fmt::layer()
        .with_ansi(atty::is(atty::Stream::Stdout))
        .with_level(true)
        .with_target(true)
        .with_thread_names(false)
        .with_file(true)
        .with_line_number(true);

    // 创建过滤器
    let filter_layer = EnvFilter::builder()
        .with_default_directive(level.into())
        .from_env_lossy();

    // 构建 subscriber
    let subscriber = Registry::default()
        .with(filter_layer)
        .with(format_layer);

    // 设置全局 subscriber
    tracing::subscriber::set_global_default(subscriber)?;

    LOGGER_INITIALIZED.set(()).map_err(|_| "Logger already initialized")?;
    Ok(())
}

/// FFI 安全的初始化函数
#[no_mangle]
pub extern "C" fn woolog_init(level: *const c_char) -> bool {
    let level_str = unsafe {
        if level.is_null() {
            "info"
        } else {
            CStr::from_ptr(level).to_str().unwrap_or("info")
        }
    };

    let level = match level_str.to_lowercase().as_str() {
        "error" => LevelFilter::ERROR,
        "warn" => LevelFilter::WARN,
        "info" => LevelFilter::INFO,
        "debug" => LevelFilter::DEBUG,
        "trace" => LevelFilter::TRACE,
        _ => LevelFilter::INFO,
    };

    init_logger(level).is_ok()
}

/// FFI 安全的日志函数
macro_rules! ffi_log_fn {
    ($name:ident, $level:ident) => {
        #[no_mangle]
        pub extern "C" fn $name(msg: *const c_char) {
            // 安全地将 C 字符串转换为 Rust 字符串
            let msg_str = unsafe {
                if msg.is_null() {
                    return;
                }
                match CStr::from_ptr(msg).to_str() {
                    Ok(s) => s,
                    Err(_) => return,
                }
            };
            
            // 使用 tracing 的 event! 宏记录日志
            event!(tracing::Level::$level, "{}", msg_str);
        }
    };
}

// 生成所有日志级别的 FFI 函数
ffi_log_fn!(woolog_trace, TRACE);
ffi_log_fn!(woolog_debug, DEBUG);
ffi_log_fn!(woolog_info, INFO);
ffi_log_fn!(woolog_warn, WARN);
ffi_log_fn!(woolog_error, ERROR);

/// Rust API (供其他 Rust 代码使用)
pub fn trace(msg: &str) {
    tracing::trace!("{}", msg);
}

pub fn debug(msg: &str) {
    tracing::debug!("{}", msg);
}

pub fn info(msg: &str) {
    tracing::info!("{}", msg);
}

pub fn warn(msg: &str) {
    tracing::warn!("{}", msg);
}

pub fn error(msg: &str) {
    tracing::error!("{}", msg);
}

/// Python 绑定模块
#[cfg(feature = "python")]
pub mod python;