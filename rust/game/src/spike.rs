use bevy::core::Timer;
use bevy::prelude::{Commands, Component, Entity, Query, Res, ResMut, With};
use cgmath::Vector3;
use dlopen::wrapper::Container;
use log::info;
use rand::Rng;
use shared::components::prefab::Prefab;
use shared::world_link::WorldLink;
use shared::data::keycode::*;
use shared::components::transform::*;

use crate::{Player, RemoveOnReset};

#[derive(Component)] pub struct YVelocity(pub f32);
#[derive(Component)] pub struct Spike;

pub fn spike_movement_system(
    time: Res<bevy::core::Time>,
    mut query: Query<(Entity, &mut Transform, &mut YVelocity), With<Spike>>)
{
    let delta = time.delta().as_secs_f32();
    for (_entity, mut transform, mut velocity) in query.iter_mut() {
        velocity.0 -= 8f32 * delta;
        transform.position.y += velocity.0 * delta;
    }
}

pub fn spike_destroy_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Spike>>)
{
    for (entity, transform) in query.iter() {
        if transform.position.y <= -2.0f32 { commands.entity(entity).despawn(); }
    }
}

pub enum SpikeSpawnerState {
    None,
    Spawning(Timer),
}

pub fn spike_spawn_system(
    mut commands: Commands,
    time: Res<bevy::core::Time>,
    mut spike_spawner_state: ResMut<SpikeSpawnerState>,
) {
    let mut spawn_timer: &mut Timer = match &mut *spike_spawner_state {
        SpikeSpawnerState::None => return,
        SpikeSpawnerState::Spawning(timer) => timer,
    };
    if !spawn_timer.tick(time.delta()).just_finished() {
        return;
    }
    // timer finished
    let spawn_x_bounds = -10f32..10f32;
    let mut rand_rng = rand::thread_rng();
    commands.spawn()
        .insert(Prefab::new("spike"))
        .insert(Spike)
        .insert(RemoveOnReset)
        .insert(Transform{position: Vector3::new(rand_rng.gen_range(spawn_x_bounds), 17f32, 0f32)})
        .insert(YVelocity(0f32));
}