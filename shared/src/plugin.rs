use std::any::Any;

pub trait Plugin: Any + Send + Sync {
    fn init(&mut self, app: &mut crate::bevy_app_syncable::App);
}

// setup dll bindings
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut Plugin {
            use shared::log_plugin::init_logger;
            init_logger();
            // make sure the constructor is the correct type.
            let constructor: fn() -> $plugin_type = $constructor;
            let object = constructor();
            let boxed: Box<Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}