use avian3d::prelude::{Collider, CollisionMargin, RigidBody};
use bevy::prelude::*;

use crate::chunk::Chunk;

#[derive(Component)]
pub struct HasCollider;

pub fn create_collider_from_mesh(
    mut commands: Commands,
    query: Query<(Entity, &Mesh3d), (With<Chunk>, Without<HasCollider>)>,
    mesh_assets: Res<Assets<Mesh>>,
) {
    for (entity, mesh) in query {
        let mesh_id = mesh.id();
        let chunk_mesh = mesh_assets.get(mesh_id).unwrap();

        let collider = commands
            .spawn((
                Collider::trimesh_from_mesh(chunk_mesh).unwrap(),
                CollisionMargin(10.0),
                Transform::from_xyz(0.0, -10.0, 0.0),
                RigidBody::Static,
                // Friction {
                //     static_coefficient: 100000.0,
                //     dynamic_coefficient: 1.0,
                //     combine_rule: CoefficientCombine::Multiply,
                // },
            ))
            .id();
        commands
            .entity(entity)
            .insert(HasCollider)
            .add_child(collider);
    }
}
