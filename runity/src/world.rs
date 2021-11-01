use std::os::raw::c_char;
use log::{debug, info};
use shared::{components::{transform::CTransform}, types::EntityId};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref WORLD: Mutex<World> = Mutex::new(World::new());
}

// communicates from GAMEDLL to UNITY, we are just the middleman
pub struct World {
    pub cb_spawn_prefab: Box<dyn Fn(*const c_char, EntityId) + Send>,
    pub cb_despawn_gameobject: Box<dyn Fn(EntityId) + Send>,
    pub cb_upload_component_transform: Box<dyn Fn(EntityId, CTransform)>,
}

impl World {
    pub fn new() -> Self {
        Self {
            cb_spawn_prefab: Box::new(|_,_|()),
            cb_despawn_gameobject: Box::new(|_|()),
            cb_upload_component_transform: Box::new(|_,_|()),
        }
    }
}

// called from unity
#[no_mangle] pub extern fn world_bind_spawn_prefab_callback(callback: extern fn(*const c_char, EntityId)) {
    debug!("binding spawn prefab, called from C# to runity.dll");
    let mut world = WORLD.lock().unwrap();
    world.cb_spawn_prefab = Box::new(move |n, i| callback(n, i));
}

#[no_mangle] pub extern fn world_bind_despawn_gameobject_callback(callback: extern fn(EntityId)) {
    debug!("binding despawn gameobject, called from C# to runity.dll");
    let mut world = WORLD.lock().unwrap();
    world.cb_despawn_gameobject = Box::new(move |id| callback(id));
}

#[no_mangle] pub extern fn world_bind_upload_component_transform_callback(callback: extern fn(EntityId, CTransform)) {
    debug!("binding upload component transform, called from C# to runity.dll");
    let mut world = WORLD.lock().unwrap();
    world.cb_upload_component_transform = Box::new(move |id, transform| callback(id, transform));
}

// FROM GAME DLL lmao
#[no_mangle] pub extern fn spawn_prefab(name: *const c_char, entity_id: EntityId) {
    debug!("calling spawn_prefab from game.dll to runity.dll");
    let world = WORLD.lock().unwrap();
    (world.cb_spawn_prefab)(name, entity_id);
}

#[no_mangle] pub extern fn despawn_gameobject(entity_id: EntityId) {
    debug!("calling despawn_gameobject from game.dll to runity.dll");
    let world = WORLD.lock().unwrap();
    (world.cb_despawn_gameobject)(entity_id);
}

#[no_mangle] pub extern fn upload_component_transform(entity_id: EntityId, transform: CTransform) {
    debug!("calling upload_component_transform from game.dll to runity.dll");
    let world = WORLD.lock().unwrap();
    debug!("did it lock?");
    (world.cb_upload_component_transform)(entity_id, transform);
}