use std::os::raw::c_char;

use dlopen;
use dlopen_derive::*;
use dlopen::wrapper::{Container, WrapperApi};

use crate::{components::{transform::CTransform}, types::EntityId};

// the game.dll conneciton to runity.dll
// is going to be a resource in the bevy ecosystem
#[derive(WrapperApi)]
pub struct WorldLink {
    spawn_prefab: fn(name: *const c_char, id: EntityId),
    despawn_gameobject: fn(id: EntityId),
    upload_component_transform: fn(id: EntityId, transform: CTransform),
}

impl WorldLink {
    pub fn new(runity_lib_path: &str) -> Result<Container<WorldLink>, dlopen::Error> {
        unsafe {Container::load(runity_lib_path)}
    }
}