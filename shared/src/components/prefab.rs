use std::{collections::{HashMap, HashSet}, ffi::CString};
use bevy::{ecs::prelude::*, prelude::Parent};

use dlopen::wrapper::Container;
use log::{debug, error, info, trace};
use crate::{components::transform::CTransform, types::EntityId, world_link::WorldLink};

use super::prelude::Transform;

#[derive(Component)]
pub struct Prefab {
    pub name: &'static str,
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
pub struct PrefabEntityTracker(pub HashMap<u64, bool>);

// tell unity to spawn the prefabs of newly created entities
pub fn sync_spawned_entitites_without_transform(
    added_entites_with_prefab: Query<(Entity, &Prefab), (Without<Transform>, Added<Prefab>)>,
    world_link: Res<Container<WorldLink>>,
    mut prefab_entity_tracker: ResMut<PrefabEntityTracker>,
) {
    for (entity, prefab) in added_entites_with_prefab.iter() {
        //let sync_id = available_entity_ids.get_id(); 
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
        trace!("telling unity to spawn prefab, id {}", entity.to_bits());
        world_link.spawn_prefab(name_ptr, entity.to_bits());
        prefab_entity_tracker.0.insert(entity.to_bits(), true);
    }
}

// tell unity to spawn the prefabs of newly created entities
pub fn sync_spawned_entitites_with_transform(
    added_entites_with_prefab: Query<(Entity, &Prefab, &Transform), (Added<Prefab>, Without<Parent>)>,
    world_link: Res<Container<WorldLink>>,
    mut prefab_entity_tracker: ResMut<PrefabEntityTracker>,
) {
    for (entity, prefab, transform) in added_entites_with_prefab.iter() {
        //let sync_id = available_entity_ids.get_id(); 
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
        trace!("telling unity to spawn prefab, id {} with transform", entity.to_bits());
        let c_transform: CTransform = (*transform).into();
        let transform_ptr = Box::into_raw(Box::new(c_transform));
        world_link.spawn_prefab_with_transform(name_ptr, entity.to_bits(), transform_ptr);
        prefab_entity_tracker.0.insert(entity.to_bits(), true);
    }
}

pub fn sync_spawned_entitites_with_transform_and_parent(
    added_entites_with_prefab: Query<(Entity, &Prefab, &Transform, &Parent), Added<Prefab>>,
    world_link: Res<Container<WorldLink>>,
    mut prefab_entity_tracker: ResMut<PrefabEntityTracker>,
) {
    for (entity, prefab, transform, parent) in added_entites_with_prefab.iter() {
        //let sync_id = available_entity_ids.get_id(); 
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
        trace!("telling unity to spawn prefab, id {} with transform", entity.to_bits());
        let c_transform: CTransform = (*transform).into();
        let transform_ptr = Box::into_raw(Box::new(c_transform));
        world_link.spawn_prefab_with_transform_and_parent(name_ptr, entity.to_bits(), transform_ptr, parent.0.to_bits());
        prefab_entity_tracker.0.insert(entity.to_bits(), true);
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
        if let Some(tracked_entity_exists) = prefab_entity_tracker.0.get_mut(&entity.to_bits()) {
            *tracked_entity_exists = true;
        }
    }
    prefab_entity_tracker.0.iter()
        .filter_map(|(entity, exists)| match exists{false => Some(entity), true => None})
        .for_each(|non_existant_entity| {
            trace!("telling unity to despawn game object with id: {}", non_existant_entity);
            world_link.despawn_gameobject(*non_existant_entity);
        });
    // remove entities that doesn't exists from tracker
    prefab_entity_tracker.0.retain(|_entity, exists| *exists);
}