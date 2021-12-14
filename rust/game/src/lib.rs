use std::time::Duration;

use bevy::core::Timer;
use bevy::reflect::List;
use cgmath::Vector3;
use log::{debug, error, info};
use rand::Rng;
use shared::components::prelude::*;
use shared::data::keycode::KeyCode;
use shared::time::Time;
use shared::plugin::{Plugin};
use shared::declare_plugin;
use bevy::ecs::prelude::*;
use bevy::app::{Events, prelude::*};
use shared::world_link::WorldLink;
use dlopen::wrapper::{Container};
pub mod particles;
mod spike;
mod gold;
mod score;
use particles::*;
use spike::*;
use gold::*;
use score::*;

struct GameState {
    time_since_start: f32,
    spawned: i32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            time_since_start: 0f32,
            spawned: 0,
        }
    }
}

#[derive(Component)] pub struct Player;
pub fn player_input_system(
    time: Res<bevy::core::Time>,
    mut query: Query<&mut Transform, With<Player>>,
    world_link: Res<Container<WorldLink>>,
) {
    let speed = 10f32;
    let mut x_input = 0;
    for mut transform in query.iter_mut() {
        if world_link.input_key_held(KeyCode::LeftArrow) {
            info!("I PRESSED THE FRICKEN LEFT BUTTON");
            x_input -= 1;
        }
        if world_link.input_key_held(KeyCode::RightArrow) {
            info!("I PRES DA FRUKKEN RITE BTN");
            x_input += 1;
        }
        transform.position.x += speed * x_input as f32 *  time.delta_seconds();  
    }
}

pub fn on_reset_level(
    mut events: EventReader<EventResetLevel>,
    query: Query<Entity, With<RemoveOnReset>>,
    mut commands: Commands,
) {
    if events.iter().count() == 0 {
        return;
    }
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// this should be slow due to world access is blocking
// not the best solution, but hey it works :P
pub fn collision_handler(
    world: &mut World,
) {
    let mut query = world.query_filtered::<&Collision, With<Prefab>>();
    let mut collected_events = Vec::new();
    for collision in query.iter(&world) {
        for event in collision.events.iter() {
            collected_events.push((event.owner_entity, event.other_entity));
        }
    }
    for event in collected_events {
        let owner_is_player = match world.get_entity_mut(Entity::from_bits(event.0)) {
            Some(owner_entity) => owner_entity.get::<Player>().is_some(),
            None => false,
        };
        let other_is_spike = match world.get_entity_mut(Entity::from_bits(event.1)) {
            Some(other_entity) => other_entity.get::<Spike>().is_some(),
            None => false,
        };
        let other_is_gold = match world.get_entity_mut(Entity::from_bits(event.1)) {
            Some(other_entity) => other_entity.get::<Gold>().is_some(),
            None => false,
        };
        if owner_is_player && other_is_spike {
            if let Some(mut events) = world.get_resource_mut::<Events<EventResetLevel>>() {
                events.send(EventResetLevel);
            }
        }
        if owner_is_player && other_is_gold {
            let new_score = {
                let mut score = world.get_resource_mut::<PlayerScore>().unwrap();
                score.0 += 1;
                info!("player score is: {}", score.0);
                score.0
            };
            for mut text in world.query_filtered::<&mut TextMeshUi, With<ScoreTag>>().iter_mut(world) {
                text.text = format!("score: {}", new_score);
            }
            // destroy the gold entity
            world.despawn(Entity::from_bits(event.1));
        }
    }
}

pub struct EventResetLevel;
#[derive(Component)] pub struct RemoveOnReset;

impl Plugin for GameState {
    fn init(&mut self, app: &mut shared::bevy_app_syncable::App) {
        // particle stuff
        app.add_system(destroy_system);
        app.add_system(spawn_system);
        app.add_system(movement_system.exclusive_system());
        app.insert_resource(CubeSpawnTimer(Timer::from_seconds(0.01f32, true)));
        app.add_event::<EventResetLevel>();
        app.add_system(on_reset_level);
        app.add_system(spike_destroy_system);
        app.add_system(spike_movement_system);
        app.add_system(spike_spawn_system);
        app.add_system(collision_handler.exclusive_system());
        app.insert_resource(SpikeSpawnerState::Spawning(Timer::from_seconds(0.6f32, true)));

        // gold
        app.add_system(gold_destroy_system);
        app.add_system(gold_movement_system);
        app.add_system(gold_spawn_system);
        app.insert_resource(GoldSpawnerState::Spawning(Timer::from_seconds(0.3f32, true)));

        // scoring
        app.add_plugin(ScorePlugin);

        // spawn player
        app.world.spawn()
            .insert(Prefab::new("player"))
            .insert(Transform{position: Vector3::new(0f32, 0f32, 0f32)})
            .insert(Collision::default())
            .insert(Player);

        app.add_system(player_input_system.exclusive_system());
    }
}

declare_plugin!(GameState, GameState::default);