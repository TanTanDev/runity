use std::{collections::{HashMap, HashSet}, ffi::CString};
use bevy::{ecs::prelude::*};

use dlopen::wrapper::Container;
use log::{debug, error, info};
use crate::{types::EntityId, world_link::WorldLink};

#[derive(Component)]
pub struct Prefab {
    pub name: &'static str,
//    pub id: Option<EntityId>, // if id is none, the unity version isn't synced
}

impl Prefab {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
//            id: None,
        }
    }
}

// could be a HashSet, but I'm using the bool as a form of reducing iteration load
// by flagging entities see, sync_despawn_prefabs
// u32: is entity id, bool: flag if entity exists
pub struct PrefabEntityTracker(pub HashMap<u32, bool>);

// tell unity to spawn the prefabs of newly created entities
pub fn sync_spawned_entitites(
    added_entites_with_prefab: Query<(Entity, &Prefab), Added<Prefab>>,
    world_link: Res<Container<WorldLink>>,
    mut prefab_entity_tracker: ResMut<PrefabEntityTracker>,
) {
    for (entity, prefab) in added_entites_with_prefab.iter() {
        //let sync_id = available_entity_ids.get_id(); 
        debug!("converting prefab name to CString");
        let name_cstr = match CString::new(prefab.name) {
            Ok(name_cstr) => name_cstr,
            Err(err) => {
                error!("failed make cstring for prefab: {:?}", err);
                continue;
            },
        };
        let name_ptr = name_cstr.as_ptr();
        // passing string to C#, so don't garbage collect
        std::mem::forget(name_cstr);
        // SEND TO UNITY
        debug!("telling unity to spawn prefab");
        world_link.spawn_prefab(name_ptr, entity.id());
        debug!("tracking entity with prefab, id: {}", entity.id());
        prefab_entity_tracker.0.insert(entity.id(), true);
    }
}

pub fn sync_despawned_entitites(
    mut prefab_entity_tracker: ResMut<PrefabEntityTracker>,
    entities_with_prefab: Query<Entity, With<Prefab>>,
    world_link: Res<Container<WorldLink>>,
) {
    // reset prefab existance flags
    for (_entity, exists) in prefab_entity_tracker.0.iter_mut() {
        *exists = false;
    }
    // find existing prefabs entities and track them 
    for entity in entities_with_prefab.iter() {
        if let Some(tracked_entity_exists) = prefab_entity_tracker.0.get_mut(&entity.id()) {
            *tracked_entity_exists = true;
        }
    }
    prefab_entity_tracker.0.iter()
        .filter_map(|(entity, exists)| match exists{false => Some(entity), true => None})
        .for_each(|non_existant_entity| {
            debug!("telling unity to despawn game object with id: {}", non_existant_entity);
            world_link.despawn_gameobject(*non_existant_entity);
        });
    // remove entities that doesn't exists from tracker
    prefab_entity_tracker.0.retain(|_entity, exists| *exists);
}