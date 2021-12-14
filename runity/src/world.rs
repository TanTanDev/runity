use std::os::raw::c_char;
use log::{debug, error, info, trace};
use shared::{components::{prelude::Collision, transform::CTransform}, data::{collision_events::CollisionEvent, keycode::KeyCode}, types::EntityId};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref WORLD: Mutex<World> = Mutex::new(World::new());
}

// communicates from GAMEDLL to UNITY, we are just the middleman
pub struct World {
    pub cb_spawn_prefab: Box<dyn Fn(*const c_char, EntityId) + Send>,
    pub cb_spawn_prefab_with_transform: Box<dyn Fn(*const c_char, EntityId, CTransform) + Send>,
    pub cb_spawn_prefab_with_transform_and_parent: Box<dyn Fn(*const c_char, EntityId, CTransform, EntityId) + Send>,
    pub cb_despawn_gameobject: Box<dyn Fn(EntityId) + Send>,
    pub cb_upload_component_transform: Box<dyn Fn(EntityId, CTransform) + Send>,
    pub cb_upload_component_textmeshui: Box<dyn Fn(EntityId, *const c_char) + Send>,
    pub cb_input_key_just_pressed: Box<dyn Fn(KeyCode) -> bool + Send>,
    pub cb_input_key_just_released: Box<dyn Fn(KeyCode) -> bool + Send>,
    pub cb_input_key_held: Box<dyn Fn(KeyCode) -> bool + Send>,
    pub cb_entity_track_collision: Box<dyn Fn(EntityId) + Send>,
}

impl World {
    pub fn new() -> Self {
        Self {
            cb_spawn_prefab: Box::new(|_,_|()),
            cb_spawn_prefab_with_transform: Box::new(|_,_,_|()),
            cb_spawn_prefab_with_transform_and_parent: Box::new(|_,_,_,_|()),
            cb_despawn_gameobject: Box::new(|_|()),
            cb_upload_component_transform: Box::new(|_,_|()),
            cb_upload_component_textmeshui: Box::new(|_,_|()),
            cb_input_key_just_pressed: Box::new(|_|false),
            cb_input_key_just_released: Box::new(|_|false),
            cb_input_key_held: Box::new(|_|false),
            cb_entity_track_collision: Box::new(|_|()),
        }
    }
}

// called from unity
#[no_mangle] pub extern fn world_bind_spawn_prefab_callback(callback: extern fn(*const c_char, EntityId)) {
    debug!("binding spawn prefab, called from C# to runity.dll");
    let mut world = WORLD.lock().unwrap();
    world.cb_spawn_prefab = Box::new(move |n, i| callback(n, i));
}

#[no_mangle] pub extern fn world_bind_spawn_prefab_with_transform_callback(callback: extern fn(*const c_char, EntityId, CTransform)) {
    debug!("binding spawn prefab with transform, called from C# to runity.dll");
    let mut world = WORLD.lock().unwrap();
    world.cb_spawn_prefab_with_transform = Box::new(move |n, i, t| callback(n, i, t));
}

#[no_mangle] pub extern fn world_bind_spawn_prefab_with_transform_and_parent_callback(callback: extern fn(*const c_char, EntityId, CTransform, EntityId)) {
    debug!("binding spawn prefab with transform and parent, called from C# to runity.dll");
    let mut world = WORLD.lock().unwrap();
    world.cb_spawn_prefab_with_transform_and_parent = Box::new(move |n, i, t, p| callback(n, i, t, p));
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

#[no_mangle] pub extern fn world_bind_upload_component_textmeshui_callback(callback: extern fn(EntityId, *const c_char)) {
    debug!("binding upload component textmeshui, called from C# to runity.dll");
    let mut world = WORLD.lock().unwrap();
    world.cb_upload_component_textmeshui = Box::new(move |id, text| callback(id, text));
}

#[no_mangle] pub extern fn world_bind_input_key_just_pressed(callback: extern fn(KeyCode) -> bool) {
    debug!("binding input key just pressed, called from C# to runity.dll");
    let mut world = WORLD.lock().unwrap();
    world.cb_input_key_just_pressed = Box::new(move |key| callback(key));
}

#[no_mangle] pub extern fn world_bind_input_key_just_released(callback: extern fn(KeyCode) -> bool) {
    debug!("binding input key just released, called from C# to runity.dll");
    let mut world = WORLD.lock().unwrap();
    world.cb_input_key_just_released = Box::new(move |key| callback(key));
}

#[no_mangle] pub extern fn world_bind_input_key_held(callback: extern fn(KeyCode) -> bool) {
    debug!("binding input key just held, called from C# to runity.dll");
    let mut world = WORLD.lock().unwrap();
    world.cb_input_key_held = Box::new(move |key| callback(key));
}

#[no_mangle] pub extern fn world_bind_entity_track_collision(callback: extern fn(EntityId)) {
    debug!("binding entity track collision, called from C# to runity.dll");
    let mut world = WORLD.lock().unwrap();
    world.cb_entity_track_collision = Box::new(move |entity_id| callback(entity_id));
}

#[no_mangle] pub extern fn world_download_entity_collision_event(entity_id: EntityId, collision_event: CollisionEvent) {
    trace!("download entity collision event, called from C# to runity.dll");
    // tell game to update collisions  
    use crate::api::*;
    let mut api = API.lock().unwrap();
    if let Some(mut entity) = api.game.app.world.get_entity_mut(bevy::ecs::entity::Entity::from_bits(entity_id)) {
        if let Some(mut collision_component) = entity.get_mut::<Collision>() {
            collision_component.events.push_back(collision_event);
        } else {
            error!("entity has no collision component wtf");
        }
    } else {error!("no entity to fetch collision component from");}
}

// FROM GAME DLL lmao
#[no_mangle] pub extern fn spawn_prefab(name: *const c_char, entity_id: EntityId) {
    trace!("calling spawn_prefab from game.dll to runity.dll");
    let world = WORLD.lock().unwrap();
    (world.cb_spawn_prefab)(name, entity_id);
}

#[no_mangle] pub extern fn spawn_prefab_with_transform(name: *const c_char, entity_id: EntityId, transform: CTransform) {
    trace!("calling spawn_prefab_with_transform from game.dll to runity.dll");
    let world = WORLD.lock().unwrap();
    (world.cb_spawn_prefab_with_transform)(name, entity_id, transform);
}

#[no_mangle] pub extern fn spawn_prefab_with_transform_and_parent(name: *const c_char, entity_id: EntityId, transform: CTransform, parent_id: EntityId) {
    trace!("calling spawn_prefab_with_transform_and_parent from game.dll to runity.dll");
    let world = WORLD.lock().unwrap();
    (world.cb_spawn_prefab_with_transform_and_parent)(name, entity_id, transform, parent_id);
}

#[no_mangle] pub extern fn despawn_gameobject(entity_id: EntityId) {
    trace!("calling despawn_gameobject from game.dll to runity.dll");
    let world = WORLD.lock().unwrap();
    (world.cb_despawn_gameobject)(entity_id);
}

#[no_mangle] pub extern fn upload_component_transform(entity_id: EntityId, transform: CTransform) {
    trace!("calling upload_component_transform from game.dll to runity.dll");
    let world = WORLD.lock().unwrap();
    (world.cb_upload_component_transform)(entity_id, transform);
}

#[no_mangle] pub extern fn upload_component_textmeshui(entity_id: EntityId, text: *const c_char) {
    trace!("calling upload_component_transform from game.dll to runity.dll");
    let world = WORLD.lock().unwrap();
    (world.cb_upload_component_textmeshui)(entity_id, text);
}

#[no_mangle] pub extern fn input_key_just_pressed(key: KeyCode) -> bool {
    trace!("calling input_key_just_pressed from game.dll to runity.dll");
    let world = WORLD.lock().unwrap();
    (world.cb_input_key_just_pressed)(key)
}

#[no_mangle] pub extern fn input_key_just_released(key: KeyCode) -> bool {
    trace!("calling input_key_just_released from game.dll to runity.dll");
    let world = WORLD.lock().unwrap();
    (world.cb_input_key_just_released)(key)
}

#[no_mangle] pub extern fn input_key_held(key: KeyCode) -> bool {
    trace!("calling input_key_held from game.dll to runity.dll");
    let world = WORLD.lock().unwrap();
    (world.cb_input_key_held)(key)
}

#[no_mangle] pub extern fn entity_track_collision(entity_id: EntityId) {
    trace!("calling entity_track_collision from game.dll to runity.dll");
    let world = WORLD.lock().unwrap();
    (world.cb_entity_track_collision)(entity_id);
}