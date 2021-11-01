use cgmath::Vector3;
use log::{debug, info};
use shared::components::prelude::*;
use shared::time::Time;
use shared::plugin::{Plugin};
use shared::declare_plugin;
use bevy::ecs::prelude::*;
use bevy::app::prelude::*;

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

#[derive(Component)]
pub struct SelfDestructTimer(f32);

fn destroy_system(
    mut commands: Commands,
    time: Res<bevy::core::Time>,
    mut query: Query<(Entity, &mut SelfDestructTimer)>)
{
    for (entity, mut timer) in query.iter_mut() {
        timer.0 -= time.delta_seconds(); 
        if timer.0 <= 0.0f32 { commands.entity(entity).despawn(); } }
}

impl Plugin for GameState {
    fn init(&mut self, app: &mut shared::bevy_app_syncable::App) {
        info!("hello from main function in game c2");
        app.add_system(destroy_system);

        app.world.spawn()
            .insert(Prefab::new("player"))
            .insert(Transform{position: Vector3::new(0.0f32, 0.0f32, 1f32)})
            .insert(SelfDestructTimer(3f32));
    }

    fn update(&mut self, time: Time, app: &mut shared::bevy_app_syncable::App) {
        self.time_since_start += time.delta_time;
        if self.time_since_start > 5.0f32 && self.spawned < 3 {
            info!("time since start {}! spawed 1 player", self.time_since_start);
            self.spawned += 1;
            if app.world.entities().len() < 3 {
                app.world.spawn()
                    .insert(Prefab::new("player"))
                    .insert(Transform{position: Vector3::new(2f32, 2f32, 2f32)})
                    .insert(SelfDestructTimer(3f32));
            }
        }
    }
}

declare_plugin!(GameState, GameState::default);