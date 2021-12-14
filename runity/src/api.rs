use std::{ffi::CStr, os::raw::c_char, sync::Mutex};

use log::{LevelFilter, debug, error, info, trace, warn};
use shared::time::Time;
use simplelog::{CombinedLogger, Config, WriteLogger};
use lazy_static::lazy_static;

use crate::{game::Game, logging::{LogMessage, UnityLogger}, world::World};

lazy_static! {
    pub static ref API: Mutex<Api> = Mutex::new(Api::new());
}


pub struct Api {
    pub world: World,
    pub game: Game,
}

impl Api {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            game: Game::new(),
        }
    }
}

#[no_mangle] pub extern fn api_init_game_lib(lib_name: *const c_char) {
    info!("api init game lib...");
    let mut api = API.lock().unwrap();

    // FIREHOSE for unity editor.
    // recreate bevy app to reset the bevy world
    api.game.app = shared::bevy_app_syncable::App::default();

    let c_str = unsafe {
        if lib_name.is_null() {
            error!("library name STR PTR is null! ABORTING LIBRARY LOADING");
            return;
        }
        CStr::from_ptr(lib_name)
    };
    let lib_name = match c_str.to_str() {
        Ok(r_str) => {
            r_str
        }, 
        Err(utf_err) => {
            error!("can't convert c# string to rust reason: {:?}\n ABORTING LIBRARY LOADING", utf_err);
            return;
        }
    };
    let library_creation_result = api.game.init_library(lib_name);
    if let Err(err) = library_creation_result {
        match err {
            crate::game::GameCreationError::DllLoadError(err) => {
                error!("{}", format!("failed to load library: {:?}. Reason: {:?}", lib_name, err).to_string());
                drop(err);
            },
        }
    }
}


#[no_mangle] pub extern fn api_init(unity_log_level: LevelFilter, writer_log_level: LevelFilter) {
    let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = vec![Box::new(UnityLogger{level: unity_log_level})];
    if writer_log_level != LevelFilter::Off {
        loggers.push(WriteLogger::new(writer_log_level, Config::default(), std::fs::File::create("my_rust_log.log").unwrap()));
    }
    let _init_result = CombinedLogger::init(loggers);
}

#[no_mangle] pub extern fn api_update(time: Time) {
    let mut api = API.lock().unwrap();
    if let Some(mut rust_plugin) = api.game.rust_plugin.take() {
        api.game.update_bevy_world();
        api.game.rust_plugin = Some(rust_plugin);
    }

    //info!("hello unity?");
    //warn!("hello unity?");
    //trace!("hello unity?");
    //debug!("hello unity?");
    //error!("hello unity?");
}  