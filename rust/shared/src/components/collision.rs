use bevy::prelude::*;
use dlopen::wrapper::Container;
use std::collections::{HashMap, VecDeque};
use crate::{data::collision_events::CollisionEvent, world_link::WorldLink};

use super::prefab::Prefab;

#[derive(Component)]
pub struct Collision {
    pub events: VecDeque<CollisionEvent>,
}

impl Default for Collision {
    fn default() -> Self {
        Self { events: VecDeque::with_capacity(2), }
    }
}

pub fn collision_clear_events_system(
    mut query: Query<&mut Collision, With<Prefab>>,
) {
    for mut collision in query.iter_mut() {
        collision.events.clear();
    }
}

pub fn sync_collision_added(
    entitiy_added_collision: Query<Entity, Added<Collision>>,
    world_link: Res<Container<WorldLink>>,
) {
    for entity in entitiy_added_collision.iter() {
        // SEND TO UNITY
        log::debug!("telling unity to track collision prefab {}", entity.to_bits());
        world_link.entity_track_collision(entity.to_bits());
    }
}