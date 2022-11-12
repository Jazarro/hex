use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::game::hex_grid::axial::{ChunkId, ColumnId};
use crate::game::hex_grid::chunks::Chunks;
use crate::game::meshes::hexagon::create_chunk_mesh;

/// Apply this component to an entity with a Transform.
/// The game will make sure chunks are loaded around the entity.
#[derive(Component, Default)]
pub struct ChunkLoader {
    pub radius_min: u32,
    pub radius_max: u32,
}

impl ChunkLoader {
    pub fn new(radius_min: u32, radius_max: u32) -> Self {
        if radius_min > radius_max {
            warn!(
                "Invalid ChunkLoader configuration! radius_min was greater than radius_max. \
                    To fix this, radius_min will be set to equal radius_max."
            );
            Self {
                radius_min: radius_max,
                radius_max,
            }
        } else {
            Self {
                radius_min,
                radius_max,
            }
        }
    }
}

/// Apply this Component to Chunk mesh entities.
/// It is used to identify which chunk a given mesh belongs to.
#[derive(Component, Default)]
pub struct ChunkMesh {
    pub id: ChunkId,
}

pub struct LoadUnloadEvent {
    pub to_be_loaded: HashSet<ChunkId>,
    pub to_be_rendered: HashSet<ChunkId>,
    pub are_rendered: HashSet<ChunkId>,
}

/// A system meant to run periodically (not every tick).
/// It checks if a chunk load / unload cycle should be triggered. If so, it triggers an event.
pub fn check_chunk_loader(
    mut events: EventWriter<LoadUnloadEvent>,
    query_loaders: Query<(&Transform, &ChunkLoader)>,
    query_mesh: Query<(Entity, &ChunkMesh)>,
) {
    debug!("Checking if we need to load any chunks.");
    let mut may_be_loaded = HashSet::default();
    let mut may_be_rendered = HashSet::default();
    let mut must_be_rendered = HashSet::default();
    for (transform, loader) in query_loaders.iter() {
        let center_chunk = ChunkId::from_xyz(&transform.translation);
        let center_chunk = ChunkId::new(center_chunk.q(), center_chunk.r(), 0);
        for qr in ColumnId::spiral(loader.radius_min).drain(0..) {
            must_be_rendered.insert(qr + center_chunk);
        }
        for qr in ColumnId::spiral(loader.radius_max).drain(0..) {
            may_be_rendered.insert(qr + center_chunk);
        }
        for qr in ColumnId::spiral(loader.radius_max + 1).drain(0..) {
            may_be_loaded.insert(qr + center_chunk);
        }
    }
    let are_rendered = query_mesh
        .iter()
        .map(|(_, chunk)| chunk.id)
        .collect::<HashSet<ChunkId>>();
    let trigger_load_unload = must_be_rendered.iter().any(|id| !are_rendered.contains(id));
    if trigger_load_unload {
        events.send(LoadUnloadEvent {
            to_be_loaded: may_be_loaded,
            to_be_rendered: may_be_rendered,
            are_rendered,
        });
    }
}

// TODO: This doesn't deal with the possibility of multiple chunks stacked on top of each other.
/// Runs when triggered by an event; only when a chunk load / unload cycle should be executed.
pub fn load_unload_chunks(
    mut commands: Commands,
    mut events: EventReader<LoadUnloadEvent>,
    mut chunks: ResMut<Chunks>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut std_mats: ResMut<Assets<StandardMaterial>>,
    query_mesh: Query<(Entity, &ChunkMesh)>,
) {
    debug!("Received LoadUnloadEvent.");
    let LoadUnloadEvent {
        to_be_loaded,
        to_be_rendered,
        are_rendered,
    } = events
        .iter()
        .last()
        .expect("This system is triggered by the event, so the event should be present.");
    // Unload chunks that don't need to be loaded:
    chunks.cull_chunks(to_be_loaded);
    // Despawn meshes that don't need to be rendered:
    for (entity, mesh) in query_mesh.iter() {
        if !to_be_rendered.contains(&mesh.id) {
            commands.entity(entity).despawn_recursive();
        }
    }
    // Load chunks that should be loaded:
    let not_yet_loaded = to_be_loaded
        .iter()
        .filter(|id| !chunks.contains(id))
        .collect::<Vec<&ChunkId>>();
    not_yet_loaded.iter().for_each(|id| {
        chunks.generate_chunk(**id);
    });
    // Create meshes for chunks that should be rendered:
    to_be_rendered
        .iter()
        .filter(|id| !are_rendered.contains(id))
        .for_each(|id| {
            commands
                .spawn_bundle(MaterialMeshBundle {
                    mesh: meshes.add(create_chunk_mesh(&chunks, id)),
                    transform: Transform::from_translation(id.center_pos().as_xyz()),
                    material: std_mats.add(Color::WHITE.into()),
                    ..default()
                })
                .insert(ChunkMesh { id: *id });
        });
}
