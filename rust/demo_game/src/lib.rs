use bevy::{prelude::{Res, Commands, IntoExclusiveSystem, Component, Query, With}, core::Time};
use dlopen::wrapper::Container;
use log::info;
use shared::{plugin::Plugin, declare_plugin, components::{prefab::Prefab, prelude::Transform}, world_link::WorldLink, data::keycode::KeyCode};
use cgmath::{Vector3, Zero};
use rand::prelude::*;

pub fn spawn_system(
    mut commands: Commands,
    monkies: Query<&Transform, With<MonkeyTag>>,
    world_link: Res<Container<WorldLink>>,
) {
    if world_link.input_key_held(KeyCode::Space) {
        let mut rng = thread_rng(); 
        for transform in monkies.iter() {
            commands.spawn()
                .insert(Prefab::new("banana"))
                .insert(Transform{position: transform.position + Vector3::new(rng.gen_range(-1f32..1f32), 0f32, rng.gen_range(-1f32..1f32))})
                .insert(Velocity(Vector3::new(rng.gen_range(-3f32..3f32), rng.gen_range(0f32..5f32), rng.gen_range(-3f32..3f32))))
                .insert(MonkeyController);
        }
    }
}

#[derive(Component)]
pub struct Velocity(pub Vector3<f32>);
pub const GRAVITY: f32 = 20f32;

pub fn velocity_system(
    time: Res<Time>,
    mut entities: Query<(&mut Transform, &mut Velocity)>,
) {
    for (mut transform, mut velocity) in entities.iter_mut() {
        velocity.0.y -= GRAVITY * time.delta_seconds();
        transform.position += velocity.0 * time.delta_seconds();
    }
}

#[derive(Component)] pub struct MonkeyController;
#[derive(Component)] pub struct MonkeyTag;

pub fn monkey_controller_system(
    time: Res<Time>,
    mut monkeys: Query<&mut Transform, With<MonkeyController>>,
    world_link: Res<Container<WorldLink>>,
) {
    let speed = 4f32;
    if world_link.input_key_held(KeyCode::W) {
        info!("tapped w");
        for mut transform in monkeys.iter_mut() {
            transform.position += Vector3::new(0f32, 0f32, 1f32) * time.delta_seconds() * speed;
        }
    }
    if world_link.input_key_held(KeyCode::S) {
        for mut transform in monkeys.iter_mut() {
            info!("moving position!");
            transform.position += Vector3::new(0f32, 0f32, -1f32) * time.delta_seconds() * speed;
        }
    }
    if world_link.input_key_held(KeyCode::A) {
        for mut transform in monkeys.iter_mut() {
            transform.position += Vector3::new(-1f32, 0f32, 0f32) * time.delta_seconds() * speed;
        }
    }
    if world_link.input_key_held(KeyCode::D) {
        for mut transform in monkeys.iter_mut() {
            transform.position += Vector3::new(1f32, 0f32, 0f32) * time.delta_seconds() * speed;
        }
    }
}

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn init(&mut self, app: &mut shared::bevy_app_syncable::App) {
        app.add_system(spawn_system.exclusive_system());
        app.add_system(monkey_controller_system.exclusive_system());
        app.add_system(velocity_system);

        app.world.spawn()
            .insert(Prefab::new("monkey"))
            .insert(Transform{position: Vector3::zero()})
            .insert(MonkeyTag)
            .insert(MonkeyController);

        app.world.spawn()
            .insert(Prefab::new("monkey"))
            .insert(Transform{position: Vector3::new(5f32, 0f32, 0f32)})
            .insert(MonkeyTag)
            .insert(MonkeyController);

        app.world.spawn()
            .insert(Prefab::new("banana"));
    }
}

declare_plugin!(GamePlugin, GamePlugin::default);