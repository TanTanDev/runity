use bevy::core::Timer;
use bevy::prelude::{Commands, Component, Entity, Query, Res, ResMut, With};
use cgmath::Vector3;
use dlopen::wrapper::Container;
use rand::Rng;
use shared::components::prefab::Prefab;
use shared::world_link::WorldLink;
use shared::data::keycode::*;
use shared::components::transform::*;

use crate::{Player, RemoveOnReset};
use crate::spike::Spike;

#[derive(Component)]
pub struct SelfDestructTimer(pub f32);
#[derive(Component)]
pub struct Velocity(pub cgmath::Vector3<f32>);

pub fn destroy_system(
    mut commands: Commands,
    time: Res<bevy::core::Time>,
    mut query: Query<(Entity, &mut SelfDestructTimer)>)
{
    for (entity, mut timer) in query.iter_mut() {
        timer.0 -= time.delta_seconds(); 
        if timer.0 <= 0.0f32 { commands.entity(entity).despawn(); } }
}

pub fn movement_system(
    time: Res<bevy::core::Time>,
    world_link: Res<Container<WorldLink>>,
    mut query: Query<(Entity, &mut Transform, &mut Velocity)>)
{
    let delta = time.delta().as_secs_f32();
    let mut rand = rand::thread_rng();
    for (_entity, mut transform, mut velocity) in query.iter_mut() {
        if world_link.input_key_held(KeyCode::Space) {
            velocity.0.y = -2f32;
            velocity.0.x += rand.gen_range(-3f32..3f32);
            velocity.0.z += rand.gen_range(-3f32..3f32);
        }
        velocity.0.y += 8f32 * delta;
        transform.position += velocity.0 * delta;
    }
}

pub struct CubeSpawnTimer(pub Timer);

pub fn spawn_system(
    mut commands: Commands,
    time: Res<bevy::core::Time>,
    player_query: Query<&Transform, With<Spike>>,
    mut cube_timer: ResMut<CubeSpawnTimer>,
) {
    if cube_timer.0.tick(time.delta()).just_finished() {
        let mut rand_rng = rand::thread_rng();
        for player_transform in player_query.iter() {
            // spawn
            commands.spawn()
                .insert(Prefab::new("particle"))
                //.insert(Transform{position: Vector3::new(rand_rng.gen_range(-2f32..2f32), 0f32, rand_rng.gen_range(-2f32..2f32))})
                .insert(Transform{position: player_transform.position})
                .insert(RemoveOnReset)
                .insert(Velocity(Vector3::new(rand_rng.gen_range(-1f32..1f32), rand_rng.gen_range(-1f32..1f32), rand_rng.gen_range(-1f32..1f32))))
                .insert(SelfDestructTimer(rand_rng.gen_range(0.3f32..19f32)));
        }
    }
}