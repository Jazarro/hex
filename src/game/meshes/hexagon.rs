use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

use crate::game::hex_grid::axial;
use crate::game::hex_grid::axial::{ChunkId, IPos, FRAC_TAU_6};
use crate::game::hex_grid::chunk::{Chunk, CHUNK_HEIGHT};
use crate::game::hex_grid::chunks::Chunks;

/// Only for testing. Maybe delete this at some point.
pub fn spawn_test_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut std_mats: ResMut<Assets<StandardMaterial>>,
) {
    for chunk_r in 0..8 {
        for chunk_q in 0..8 {
            let chunk_pos = ChunkId::new(chunk_q as i32, chunk_r as i32, 0);
            // info!("Chunk: {:?}, center at: {:?}", chunk_pos, chunk_pos.center_pos());
            commands.spawn_bundle(MaterialMeshBundle {
                mesh: meshes.add(create_single_block_mesh()),
                transform: Transform::from_translation(
                    chunk_pos.center_pos().as_xyz()
                        + Vec3::new(0., 0., 1. + (chunk_q + chunk_r) as f32 * 0.5),
                ),
                material: std_mats.add(Color::WHITE.into()),
                ..default()
            });

            for block_pos in Chunk::chunk_columns()
                .iter()
                .map(|relative_pos| relative_pos + chunk_pos.center_pos())
            {
                let mut xyz = block_pos.as_xyz();
                xyz.z += if chunk_q % 2 == 0 { 0. } else { 0.1 };
                xyz.z += (chunk_q + chunk_r) as f32 * 0.5;
                let what_chunk_it_thinks_it_belongs_to = ChunkId::from_block_pos(&block_pos);
                commands.spawn_bundle(MaterialMeshBundle {
                    mesh: meshes.add(create_single_block_mesh()),
                    transform: Transform::from_translation(xyz),
                    material: std_mats.add(
                        Color::rgb(
                            (what_chunk_it_thinks_it_belongs_to.q() as f32 / 8.).abs(),
                            (what_chunk_it_thinks_it_belongs_to.r() as f32 / 8.).abs(),
                            0.,
                        )
                        .into(),
                    ),
                    ..default()
                });
            }
        }
    }
}

pub fn create_chunk_mesh(chunks: &Chunks, chunk_id: &ChunkId) -> Mesh {
    let chunk = chunks.get_chunk(chunk_id);
    let vertical_neighbours = [
        IPos::new(0, 0, -1), // <== Bottom neighbour
        IPos::new(0, 0, 1),  // <== Top neighbour
    ];
    let vertical_normals = [
        Vec3::new(0., 0., -1.), // <== Bottom normal
        Vec3::new(0., 0., 1.),  // <== Top normal
    ];
    let mut vertices = vec![];
    let mut indices = vec![];
    for pos in Chunk::chunk_columns().iter() {
        // First add all the vertical faces:
        (0..6).for_each(|i: i8| {
            let angle_a = FRAC_TAU_6 * i as f32;
            let angle_b = FRAC_TAU_6 * (i + 1).rem_euclid(6) as f32;
            let normal_a = Vec3::new(angle_a.cos(), angle_a.sin(), 0.);
            let normal_b = Vec3::new(angle_b.cos(), angle_b.sin(), 0.);
            let normal_face = ((normal_a + normal_b) / 2.).normalize();
            for z in 0..CHUNK_HEIGHT {
                let pos_relative = pos.as_ipos(z as i32);
                if !chunk.block(&pos_relative).is_solid() {
                    // This block isn't solid, so we obviously shouldn't include it in the mesh.
                    continue;
                }
                let pos_absolute = pos_relative + chunk_id.center_pos();
                let neighbour = pos_absolute.neighbour(i as u32);
                if chunks.is_solid(&neighbour) {
                    // The neighbour is solid, so there is no point in rendering this face.
                    continue;
                }
                let (pos_a_bottom, pos_a_top) = calc_pos(angle_a, &pos_relative);
                let (pos_b_bottom, pos_b_top) = calc_pos(angle_b, &pos_relative);
                vertices.push((pos_a_bottom, normal_face, [1., 1.]));
                vertices.push((pos_b_bottom, normal_face, [1., 1.]));
                vertices.push((pos_a_top, normal_face, [1., 1.]));
                vertices.push((pos_b_top, normal_face, [1., 1.]));
                let len = vertices.len() as u32;
                indices.append(&mut vec![len - 4, len - 3, len - 2]);
                indices.append(&mut vec![len - 1, len - 2, len - 3]);
            }
        });
        // Now add the top and bottom faces:
        for z in 0..CHUNK_HEIGHT {
            let pos_relative = pos.as_ipos(z as i32);
            if !chunk.block(&pos_relative).is_solid() {
                // This block isn't solid, so we obviously shouldn't include it in the mesh.
                continue;
            }
            let pos_absolute = pos_relative + chunk_id.center_pos();
            (0..2).for_each(|j: i8| {
                if j==0 && pos_absolute.z() ==0 {
                    // Do not draw the bottom of the world, it's never seen.
                    return;
                }
                // j==0 for bottom face, j==1 for top face.
                // Check if the neighbour is solid. If so, we don't have to render this face:
                let neighbour = pos_absolute + vertical_neighbours[j as usize];
                if !chunks.is_solid(&neighbour) {
                    let xyz = pos_relative.delta(0, 0, j as i32).as_xyz();
                    let len = vertices.len() as u32;
                    // Center vertex:
                    vertices.push((xyz, vertical_normals[j as usize], [1., 1.]));
                    // Corner vertices:
                    (0..6).for_each(|i: i8| {
                        let angle = FRAC_TAU_6 * i as f32;
                        let pos_corner = Vec3::new(
                            angle.cos() * axial::RADIUS + xyz.x,
                            angle.sin() * axial::RADIUS + xyz.y,
                            xyz.z,
                        );
                        vertices.push((pos_corner, vertical_normals[j as usize], [1., 1.]));
                        indices.append(&mut vec![
                            len,
                            len + 1 + (i as u32 + 1 - j as u32).rem_euclid(6),
                            len + 1 + (i as u32 + j as u32).rem_euclid(6),
                        ]);
                    });
                }
            });
        }
    }
    debug!(
        "Spawned chunk ({},{},{}) with {} vertices and {} indices.",
        chunk_id.q(),
        chunk_id.r(),
        chunk_id.z(),
        vertices.len(),
        indices.len()
    );
    let positions: Vec<_> = vertices.iter().map(|(p, _, _)| p.to_array()).collect();
    let normals: Vec<_> = vertices.iter().map(|(_, n, _)| n.to_array()).collect();
    let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}

pub fn create_single_block_mesh() -> Mesh {
    let normal_bottom = Vec3::new(0.0, 0.0, -1.0);
    let normal_top = Vec3::new(0.0, 0.0, 1.0);
    let vertex_bottom = (Vec3::ZERO, normal_bottom, [1.0, 1.0]);
    let vertex_top = (Vec3::new(0., 0., axial::HEIGHT), normal_top, [1.0, 1.0]);
    let mut vertices = vec![vertex_bottom, vertex_top];
    let mut indices = vec![];
    (0..6).for_each(|i: i8| {
        let angle_a = (std::f32::consts::TAU / 6.) * i as f32;
        let angle_b = (std::f32::consts::TAU / 6.) * (i + 1).rem_euclid(6) as f32;
        let normal_a = Vec3::new(angle_a.cos(), angle_a.sin(), 0.);
        let normal_b = Vec3::new(angle_b.cos(), angle_b.sin(), 0.);
        let normal_face = ((normal_a + normal_b) / 2.).normalize();
        let (pos_a_bottom, pos_a_top) = calc_pos(angle_a, &IPos::default());
        let (pos_b_bottom, pos_b_top) = calc_pos(angle_b, &IPos::default());
        // Add vertices. In order to get sharp edges, add each vertex thrice: once per face.
        vertices.push((pos_a_bottom, normal_face, [1., 1.]));
        vertices.push((pos_b_bottom, normal_face, [1., 1.]));
        vertices.push((pos_a_top, normal_face, [1., 1.]));
        vertices.push((pos_b_top, normal_face, [1., 1.]));
        vertices.push((pos_a_bottom, normal_bottom, [1., 1.]));
        vertices.push((pos_b_bottom, normal_bottom, [1., 1.]));
        vertices.push((pos_a_top, normal_top, [1., 1.]));
        vertices.push((pos_b_top, normal_top, [1., 1.]));
        // Two face triangles:
        indices.append(&mut vec![index(i, 0), index(i, 1), index(i, 2)]);
        indices.append(&mut vec![index(i, 3), index(i, 2), index(i, 1)]);
        // Bottom triangle:
        indices.append(&mut vec![index(i, 4), 0, index(i, 5)]);
        // Top triangle:
        indices.append(&mut vec![index(i, 6), index(i, 7), 1]);
    });

    let positions: Vec<_> = vertices.iter().map(|(p, _, _)| p.to_array()).collect();
    let normals: Vec<_> = vertices.iter().map(|(_, n, _)| n.to_array()).collect();
    let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}

fn index(i: i8, local_index: i8) -> u32 {
    2 + i as u32 * 8 + local_index as u32
}

fn calc_pos(angle: f32, pos: &IPos) -> (Vec3, Vec3) {
    let xyz = pos.as_xyz();
    let pos_bottom = Vec3::new(
        angle.cos() * axial::RADIUS + xyz.x,
        angle.sin() * axial::RADIUS + xyz.y,
        xyz.z,
    );
    let pos_top = Vec3::new(
        angle.cos() * axial::RADIUS + xyz.x,
        angle.sin() * axial::RADIUS + xyz.y,
        axial::HEIGHT + xyz.z,
    );
    (pos_bottom, pos_top)
}
