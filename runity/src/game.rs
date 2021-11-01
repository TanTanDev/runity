use dlopen::{self, symbor::RefMut};
use dlopen_derive::*;
use dlopen::wrapper::{Container, WrapperApi};
use dlopen::symbor::SymBorApi;
use log::{debug, error, info};
use shared::bevy_app_syncable::App;
use shared::components::transform::upload_component_transform_system;
use shared::world_link::WorldLink;
use shared::{plugin::{self, Plugin}, time::Time};
use shared::components::prelude::*;
use std::collections::HashMap;

use bevy::ecs::prelude::*;
use bevy::app::prelude::*;

#[derive(Debug)]
pub enum GameCreationError {
    //LibLoadingError(libloading::Error),
    DllLoadError(dlopen::Error),
}

// communication with game specific dll
pub struct Game {
    pub rust_connection: Option<Container<RustConnection>>,
    // pointer to the game defined plugin struct
    pub rust_plugin: Option<Box<dyn Plugin>>,
    //pub world: World,
    //pub schedule: Schedule,
    pub app: shared::bevy_app_syncable::App,
}

// the game conneciton
#[derive(WrapperApi)]
pub struct RustConnection {
    _plugin_create: extern "C" fn() -> *mut dyn Plugin,
}

impl RustConnection {
    pub fn new(lib_name: &str) -> Result<Container<Self>, GameCreationError> {
        let cont_result: Result<Container<RustConnection>, dlopen::Error> = unsafe { Container::load(lib_name)};
        let container = match cont_result {
            Ok(container) => container,
            Err(err) => return Err(GameCreationError::DllLoadError(err)),
        };
        //container.init();
        Ok(container)
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            rust_connection: None,
            rust_plugin: None,
            app: App::default(),
        }
    }

    pub fn setup_bevy_world(&mut self) {
        //self.app.add_system_to_stage(shared::core_stage::CoreStage::UploadToUnity, sync_spawned_entitites.label("s_s_e"));
        //self.app.add_system_to_stage(shared::core_stage::CoreStage::UploadToUnity, sync_despawned_entitites.label("s_d_e").after("s_s_e"));
        self.app.add_system_to_stage(shared::core_stage::CoreStage::UploadToUnity, upload_component_transform_system.label("u_c_t"));
        match WorldLink::new("runity.dll") {
            Ok(world_link) => {
                self.app.insert_resource(world_link);
            },
            Err(err) => {
                error!("failed to initialize world_link, fatal! : {:?}", err);
            }
        }
        self.app.insert_resource(PrefabEntityTracker(HashMap::with_capacity(2)));
        use bevy::core::Time;
        let mut time = Time::default();
        // 0 first frame delta_time
        time.update();
        self.app.insert_resource(time);
        self.app.add_system_to_stage(shared::core_stage::CoreStage::First, (|mut time: ResMut<Time>| time.update()).exclusive_system());
    }

    pub fn update_bevy_world(&mut self) {
        //self.schedule.run(&mut self.wo);
        self.app.update();
    }

    pub fn init_library(&mut self, lib_name: &str) -> Result<(), GameCreationError> {
        if self.rust_plugin.is_some() {
            error!("RUST PLUGIN IS ALREADY LOADED OMGGGG");
        }
        match RustConnection::new(lib_name) {
            Ok(rust_connection) => {
                debug!("setting up bevy world");
                self.setup_bevy_world();
                debug!("creating game dll link");
                let plugin_ptr = rust_connection._plugin_create();
                let mut rust_plugin = unsafe{Box::from_raw(plugin_ptr)};
                debug!("calling game.init()");
                // app builder should exist by now, unwrap
                rust_plugin.init(&mut self.app);
                self.rust_plugin = Some(rust_plugin);
                self.rust_connection = Some(rust_connection);
                Ok(())
            },
            Err(err) => {
                Err(err)
            },
        }
    }
}