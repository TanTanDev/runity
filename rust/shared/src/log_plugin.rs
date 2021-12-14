use log::{Level, LevelFilter, Log, Metadata, Record};
use dlopen;
use dlopen_derive::*;
use dlopen::wrapper::{Container, WrapperApi};
use simplelog::{CombinedLogger, Config, SharedLogger};
use lazy_static::lazy_static;
use std::{ffi::CString, os::raw::c_char, sync::Mutex};

lazy_static!{
    pub static ref LOG_API: Mutex<Option<Container<LogApi>>> = Mutex::new(None);
}


// the game conneciton
#[derive(WrapperApi)]
pub struct LogApi {
    warn: fn(message: *const c_char),
    info: fn(message: *const c_char),
    error: fn(message: *const c_char),
    debug: fn(message: *const c_char),
    trace: fn(message: *const c_char),
}

impl LogApi {
    pub fn new(runity_lib_path: &str) -> Result<Container<LogApi>, dlopen::Error> {
        let cont_result: Result<Container<LogApi>, dlopen::Error> = unsafe { Container::load(runity_lib_path)};
        let container = match cont_result {
            Ok(container) => container,
            Err(err) => return Err(err),
        };
        Ok(container)
    }
}

// used by game dll do communicate back to the rust runity ap
struct GameDllLogger;
impl log::Log for GameDllLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            //println!("{} - {}", record.level(), record.args());
            let log_api_option = LOG_API.lock().unwrap();
            if let Some(log_api) = &*log_api_option {
                let message = CString::new(format!("[GAME] {}", record.args().to_string())).unwrap();
                let message = message.as_ptr();
                match record.level() {
                    Level::Error => log_api.error(message),
                    Level::Warn => log_api.warn(message),
                    Level::Info => log_api.info(message),
                    Level::Debug => log_api.debug(message),
                    Level::Trace => log_api.trace(message),
                }
            }
        }
    }

    fn flush(&self) {}
}

impl SharedLogger for GameDllLogger {
    fn level(&self) -> LevelFilter {
        LevelFilter::Debug
    }
    fn config(&self) -> Option<&Config> {
        None//(Some(&self.config)
    }
    fn as_log(self: Box<Self>) -> Box<dyn Log> {
        Box::new(*self)
    }
}

pub fn init_logger() {
    let mut log_api_option = LOG_API.lock().unwrap();
    let log_api_result = LogApi::new("runity.dll");
    if let Ok(log_api) = log_api_result {
        *log_api_option = Some(log_api);
    }

    let _init_result = CombinedLogger::init(vec![
        Box::new(GameDllLogger),
    ]);
}