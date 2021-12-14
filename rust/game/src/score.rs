use bevy::prelude::{BuildWorldChildren, Component, EventReader, Query, ResMut, With};
use cgmath::Vector3;
use shared::{bevy_app_syncable::App, bevy_plugin_syncable::Plugin, components::{prefab::Prefab, prelude::{TextMeshUi, Transform}}};

use crate::EventResetLevel;

pub struct PlayerScore(pub i32);

pub fn score_reset_system(
    mut events: EventReader<EventResetLevel>,
    mut player_score: ResMut<PlayerScore>,
    mut text_query: Query<&mut TextMeshUi, With<ScoreTag>>,
) {
    if events.iter().count() > 0 {
        player_score.0 = 0; 
        for mut text in text_query.iter_mut() {
            text.text = format!("score: {}", player_score.0);
        }
    }
}

pub struct ScorePlugin;
#[derive(Component)]
pub struct ScoreTag;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        // spawn canvas will score
        app.world.spawn()
            .insert(Prefab::new("canvas"))
            .with_children(|parent| {
                parent.spawn()
                    .insert(Prefab::new("score_ui"))
                    .insert(Transform{position: Vector3::new(0.,362.,0.)})
                    .insert(TextMeshUi::new("score: X".to_string()))
                    .insert(ScoreTag);
            });

        app.insert_resource(PlayerScore(0));
        app.add_system(score_reset_system);
    }
}