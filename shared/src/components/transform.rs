use bevy::ecs::prelude::*;
use dlopen::wrapper::Container;
use log::debug;
use crate::world_link::WorldLink;

use super::prefab::Prefab;
use cgmath::{Vector3};

#[derive(Component, Copy, Clone, Debug)]
pub struct Transform {
    pub position: Vector3<f32>,
} 

impl Into<CTransform> for Transform {
    fn into(self) -> CTransform {
        CTransform {
  //          position: CVector{
                x: self.position.x as i32,
                y: self.position.y as i32,
                z: self.position.x as i32,
   //         },
        }
    }
}

#[repr(C)]
//#[derive(Copy, Clone, Debug)]
#[derive(Debug)]
pub struct CTransform {
    pub x: i32,
    pub y: i32,
    pub z: i32,
} 

// #[repr(C)]
// #[derive(Copy, Clone, Debug)]
// pub struct CVector {
//     pub x: f32,
//     pub y: f32,
//     pub z: f32,
// } 

fn download_unity_transform(
    mut transforms: Query<(&mut Prefab, &mut Transform)>,
) {
    for (prefab, transform) in transforms.iter_mut() {
    }
}

pub fn upload_component_transform_system(
    prefab_with_transform: Query<(Entity, &Transform), With<Prefab>>,
    world_link: Res<Container<WorldLink>>,
) {
    // reset prefab existance flags
    for (entity, transform) in prefab_with_transform.iter() {
        debug!("telling unity to sync transform with gameobject id: {}", entity.id());
        let test: CTransform = (*transform).into();
        debug!("transform data: {:?}", test);
        let transform: CTransform = (*transform).into();
        world_link.upload_component_transform(entity.id(), transform);
        debug!("we survived?");
    }
}