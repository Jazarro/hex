use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

use crate::game::hex_grid::axial;
use crate::game::hex_grid::axial::{IPos, Pos, FRAC_TAU_6};
use crate::game::hex_grid::block::BlockType;
use crate::game::hex_grid::chunk::{
    Chunk, CHUNK_DIMENSION_Q, CHUNK_DIMENSION_R, CHUNK_DIMENSION_Z,
};
use crate::game::hex_grid::chunks::Chunks;

/// For testing.
///
/// Old, unoptimised scenario:
/// For a 64 x 64 x 16 chunk, this uses 1308500 vertices across 26170 meshes.
///
pub fn spawn_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut std_mats: ResMut<Assets<StandardMaterial>>,
) {
    let chunk = Chunk::new(IVec2::new(0, 0));
    for z in 0..CHUNK_DIMENSION_Z - 1 {
        for r in 0..CHUNK_DIMENSION_R - 1 {
            for q in 0..CHUNK_DIMENSION_Q - 1 {
                let pos = Pos::new(q as f32, r as f32, z as f32);

                let block = chunk.get_by_qrz(q, r, z);
                if block.block_type == BlockType::Air {
                    continue;
                }

                commands.spawn_bundle(MaterialMeshBundle {
                    mesh: meshes.add(create_single_block_mesh()),
                    transform: Transform::from_translation(pos.as_xyz()),
                    material: std_mats.add(
                        // Color::rgb(
                        //     q as f32 / chunk::CHUNK_DIMENSION_Q as f32,
                        //     r as f32 / chunk::CHUNK_DIMENSION_R as f32,
                        //     z as f32 / chunk::CHUNK_DIMENSION_Z as f32,
                        // )
                        // .into(),
                        Color::WHITE.into(),
                    ),
                    ..default()
                });
            }
        }
    }
}

pub fn spawn_chunk_new(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut std_mats: ResMut<Assets<StandardMaterial>>,
) {
    let mut chunks = Chunks::default();
    let chunk_pos = IPos::splat(0);
    chunks.generate_chunk(chunk_pos);
    commands.spawn_bundle(MaterialMeshBundle {
        mesh: meshes.add(create_chunk_mesh(&chunks, &chunk_pos)),
        transform: Transform::default(),
        material: std_mats.add(
            // Color::rgb(
            //     q as f32 / chunk::CHUNK_DIMENSION_Q as f32,
            //     r as f32 / chunk::CHUNK_DIMENSION_R as f32,
            //     z as f32 / chunk::CHUNK_DIMENSION_Z as f32,
            // )
            // .into(),
            Color::WHITE.into(),
        ),
        ..default()
    });
}

pub fn create_chunk_mesh(chunks: &Chunks, chunk_pos: &IPos) -> Mesh {
    let chunk = chunks.get_chunk(chunk_pos);
    // On the unit circle, the vertex at i=0 is located at (1,0).
    // Vertices are going counter-clockwise around the unit circle.
    // The first neighbour at (q=1, r=1) borders the face between the first two vertices.
    let neighbours_horizontal = [
        IPos::new(1, 0, 0),  // <== North-east neighbour
        IPos::new(0, 1, 0),  // <== North neighbour.
        IPos::new(-1, 1, 0), // <== North-west neighbour.
        IPos::new(-1, 0, 0), // <== South-west neighbour.
        IPos::new(0, -1, 0), // <== South neighbour.
        IPos::new(1, -1, 0), // <== South-east neighbour.
    ];
    let vertical_neighbours = [
        IPos::new(0, 0, -1), // <== Bottom neighbour
        IPos::new(0, 0, 1),  // <== Top neighbour
    ];
    let vertical_normals = [
        Vec3::new(0., 0., -1.), // <== Bottom normal
        Vec3::new(0., 0., 1.),  // <== Top normal
    ];
    let normal_bottom = Vec3::new(0.0, 0.0, -1.0);
    let normal_top = Vec3::new(0.0, 0.0, 1.0);
    let mut vertices = vec![];
    let mut indices = vec![];
    for q in 0..CHUNK_DIMENSION_Q {
        for r in 0..CHUNK_DIMENSION_R {
            // First add all the vertical faces:
            (0..6).for_each(|i: i8| {
                let angle_a = FRAC_TAU_6 * i as f32;
                let angle_b = FRAC_TAU_6 * (i + 1).rem_euclid(6) as f32;
                let normal_a = Vec3::new(angle_a.cos(), angle_a.sin(), 0.);
                let normal_b = Vec3::new(angle_b.cos(), angle_b.sin(), 0.);
                let normal_face = ((normal_a + normal_b) / 2.).normalize();
                for z in 0..CHUNK_DIMENSION_Z {
                    let pos_relative = IPos::new(q as i32, r as i32, z as i32);
                    if !chunk.get(&pos_relative).is_solid() {
                        // This block isn't solid, so we obviously shouldn't include it in the mesh.
                        continue;
                    }
                    let pos_absolute = &pos_relative + chunk_pos;
                    let neighbour = pos_absolute + neighbours_horizontal[i as usize];
                    if chunks.is_solid(&neighbour) {
                        // The neighbour is solid, so there is no point in rendering this face.
                        continue;
                    }
                    let (pos_a_bottom, pos_a_top) = calc_pos(angle_a, &pos_absolute);
                    let (pos_b_bottom, pos_b_top) = calc_pos(angle_b, &pos_absolute);
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
            for z in 0..CHUNK_DIMENSION_Z {
                let pos_relative = IPos::new(q as i32, r as i32, z as i32);
                if !chunk.get(&pos_relative).is_solid() {
                    // This block isn't solid, so we obviously shouldn't include it in the mesh.
                    continue;
                }
                let pos_absolute = &pos_relative + chunk_pos;
                (0..2).for_each(|j: i8| {
                    // j==0 for bottom face, j==1 for top face.
                    // Check if the neighbour is solid. If so, we don't have to render this face:
                    let neighbour = pos_absolute + vertical_neighbours[j as usize];
                    if !chunks.is_solid(&neighbour) {
                        let xyz = pos_absolute.delta(0, 0, j as i32).as_xyz();
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
    }
    debug!(
        "Spawned chunk ({},{},{}) with {} vertices and {} indices.",
        chunk_pos.q(),
        chunk_pos.r(),
        chunk_pos.z(),
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
