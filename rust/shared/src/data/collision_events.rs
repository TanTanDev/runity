use bevy::prelude::*;

use crate::types::EntityId;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CollisionEvent {
    pub owner_entity: EntityId,
    pub other_entity: EntityId,
    pub collision_type: CollisionType,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CollisionType {
    OnCollisionEnter,
    OnCollisionExit,
    OnCollisionStay,
    OnTriggerEnter,
    OnTriggerExit,
    OnTriggerStay,
}