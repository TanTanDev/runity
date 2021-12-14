use std::os::raw::c_char;

use dlopen;
use dlopen_derive::*;
use dlopen::wrapper::{Container, WrapperApi};

use crate::{components::{transform::CTransform}, data::keycode::KeyCode, types::EntityId};

// the game.dll conneciton to runity.dll
// is going to be a resource in the bevy ecosystem
#[derive(WrapperApi)]
pub struct WorldLink {
    spawn_prefab: fn(name: *const c_char, id: EntityId),
    spawn_prefab_with_transform: fn(name: *const c_char, id: EntityId, transform: *mut CTransform),
    spawn_prefab_with_transform_and_parent: fn(name: *const c_char, id: EntityId, transform: *mut CTransform, parent_id: EntityId),
    despawn_gameobject: fn(id: EntityId),
    upload_component_transform: fn(id: EntityId, transform: *mut CTransform),
    upload_component_textmeshui: fn(id: EntityId, text: *const c_char),
    input_key_just_pressed: fn(key: KeyCode) -> bool,
    input_key_just_released: fn(key: KeyCode) -> bool,
    input_key_held: fn(key: KeyCode) -> bool,
    entity_track_collision: fn(id: EntityId),
}

impl WorldLink {
    pub fn new(runity_lib_path: &str) -> Result<Container<WorldLink>, dlopen::Error> {
        unsafe {Container::load(runity_lib_path)}
    }
}