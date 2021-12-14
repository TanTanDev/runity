use log::{Level, LevelFilter, Log, Metadata, Record};
use std::{ffi::{CStr, CString}, os::raw::c_char, sync::Mutex};
use simplelog::{Config, SharedLogger};
use lazy_static::lazy_static;

lazy_static! {
    static ref UNITY_LOGGER_CALLBACK: Mutex<UnityLoggerCallback> = Mutex::new(UnityLoggerCallback::new());
}

// C# replica
#[repr(C)]
pub struct LogMessage {
    level: LevelFilter,
    message: *const c_char,
}

impl LogMessage {
    pub fn new(record: &Record) -> Self {
        let c_message = CString::new(record.args().to_string()).unwrap();
        let p_message = c_message.as_ptr();
        std::mem::forget(c_message);
        Self {
            level: record.level().to_level_filter(),
            message: p_message,
        }
    }
}

// calls into unity
struct UnityLoggerCallback {
    pub cb_log: Box<dyn Fn(LogMessage) + Send>,
}

impl UnityLoggerCallback {
    pub fn new() -> Self {
        Self {
            cb_log: Box::new(|_|()),
        }
    }
}

// called from unity
#[no_mangle] pub extern fn bind_log_callback(callback: extern fn(LogMessage)) {
    let mut u_logger = UNITY_LOGGER_CALLBACK.lock().unwrap();
    u_logger.cb_log = Box::new(move |log_msg| callback(log_msg));
}

// rust side Log implementation
pub struct UnityLogger {
    pub level: LevelFilter,
}

impl log::Log for UnityLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()){
            return;
        }

        // todo: SHOULD NOT TRY_LOCK, But have to because function within API, can't log currently
        if let Ok(u_logger) = UNITY_LOGGER_CALLBACK.try_lock() {
            (u_logger.cb_log)(LogMessage::new(record));
        }
    }
    fn flush(&self) { }
}

impl SharedLogger for UnityLogger {
    fn level(&self) -> LevelFilter {
        self.level
    }
    fn config(&self) -> Option<&Config> {
        None//(Some(&self.config)
    }
    fn as_log(self: Box<Self>) -> Box<dyn Log> {
        Box::new(*self)
    }
}


// used from game dll
#[no_mangle] pub extern fn warn(message: *const c_char) {
    let message = unsafe {CStr::from_ptr(message)};
    if let Ok(message) = message.to_str() {
        log::warn!("{}", message);
    }
}

#[no_mangle] pub extern fn info(message: *const c_char) {
    let message = unsafe {CStr::from_ptr(message)};
    if let Ok(message) = message.to_str() {
        log::info!("{}", message);
    }
}

#[no_mangle] pub extern fn debug(message: *const c_char) {
    let message = unsafe {CStr::from_ptr(message)};
    if let Ok(message) = message.to_str() {
        log::debug!("{}", message);
    }
}

#[no_mangle] pub extern fn trace(message: *const c_char) {
    let message = unsafe {CStr::from_ptr(message)};
    if let Ok(message) = message.to_str() {
        log::info!("{}", message);
    }
}

#[no_mangle] pub extern fn error(message: *const c_char) {
    let message = unsafe {CStr::from_ptr(message)};
    if let Ok(message) = message.to_str() {
        log::error!("{}", message);
    }
}