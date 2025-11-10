use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

pub fn create_collider_from_mesh(
    mut commands: Commands,
    query: Query<(Entity, &Mesh3d), Without<Collider>>,
    mesh_assets: Res<Assets<Mesh>>,
) {
    for (entity, mesh) in query {
        let mesh_id = mesh.id();
        let chunk_mesh = mesh_assets.get(mesh_id).unwrap();
        commands.entity(entity).insert((
            RigidBody::Static,
            Collider::trimesh_from_mesh(chunk_mesh).unwrap(),
        ));
    }
}
