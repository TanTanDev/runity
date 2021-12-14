use bevy::prelude::*;
use dlopen::wrapper::Container;

use crate::{utils, world_link::WorldLink};

#[derive(Component)]
pub struct TextMeshUi {
    pub text: String,
}

impl TextMeshUi {
    pub fn new(text: String) -> Self {
        Self {
            text,
        }
    }
}

pub fn upload_component_textmeshui_system(
    entitiy_added_collision: Query<(Entity, &TextMeshUi), Changed<TextMeshUi>>,
    world_link: Res<Container<WorldLink>>,
) {
    for (entity, text_mesh_ui) in entitiy_added_collision.iter() {
        // SEND TO UNITY
        log::debug!("telling unity to update textmeshui, entity: {}", entity.to_bits());
        if let Ok(text_str) = utils::string_to_cstring_ptr(text_mesh_ui.text.as_str()) {
            world_link.upload_component_textmeshui(entity.to_bits(), text_str);
        }
    }
}