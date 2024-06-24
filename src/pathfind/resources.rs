use bevy::prelude::*;
use vleue_navigator::NavMesh;

#[derive(Resource)]
pub struct Handles(pub Handle<Gltf>, pub Option<Handle<NavMesh>>);
