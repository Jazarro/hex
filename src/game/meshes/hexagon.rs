use crate::game::hex_grid::axial;
use crate::game::hex_grid::axial::Pos;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

pub fn spawn_hex(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut std_mats: ResMut<Assets<StandardMaterial>>,
) {
    let (length, width) = (10, 10);
    (0..width).for_each(|z| {
        (0..length).for_each(|x| {
            let pos = Pos::new(x as f32, z as f32, 0.);
            commands.spawn_bundle(MaterialMeshBundle {
                mesh: meshes.add(create_mesh()),
                transform: Transform::from_translation(pos.to_pixel()),
                material: std_mats
                    .add(Color::rgb(0.5, x as f32 / length as f32, z as f32 / width as f32).into()),
                ..default()
            });
        });
    });
}

#[derive(Debug, Copy, Clone)]
pub struct HexBlock {
    pub height: f32,
    /// Long radius, ie the distance between center and corner point.
    pub radius: f32,
}

impl Default for HexBlock {
    fn default() -> Self {
        HexBlock {
            height: 0.5,
            radius: 0.25,
        }
    }
}

fn index(i: i8, local_index: i8) -> u32 {
    2 + i as u32 * 8 + local_index as u32
}

fn calc_pos(angle: f32) -> (Vec3, Vec3) {
    let pos_bottom = Vec3::new(angle.cos() * axial::RADIUS, 0., angle.sin() * axial::RADIUS);
    let pos_top = Vec3::new(
        angle.cos() * axial::RADIUS,
        axial::HEIGHT,
        angle.sin() * axial::RADIUS,
    );
    (pos_bottom, pos_top)
}

fn create_mesh() -> Mesh {
    let normal_bottom = Vec3::new(0.0, -1.0, 0.0);
    let normal_top = Vec3::new(0.0, 1.0, 0.0);
    let vertex_bottom = (Vec3::ZERO, normal_bottom, [1.0, 1.0]);
    let vertex_top = (Vec3::new(0., axial::HEIGHT, 0.), normal_top, [1.0, 1.0]);
    let mut vertices = vec![vertex_bottom, vertex_top];
    let mut indices = vec![];
    (0..6).for_each(|i: i8| {
        let angle_a = (std::f32::consts::TAU / 6.) * i as f32;
        let angle_b = (std::f32::consts::TAU / 6.) * (i + 1).rem_euclid(6) as f32;
        let normal_a = Vec3::new(angle_a.cos(), 0., angle_a.sin());
        let normal_b = Vec3::new(angle_b.cos(), 0., angle_b.sin());
        let normal_face = ((normal_a + normal_b) / 2.).normalize();
        let (pos_a_bottom, pos_a_top) = calc_pos(angle_a);
        let (pos_b_bottom, pos_b_top) = calc_pos(angle_b);
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
        indices.append(&mut vec![index(i, 0), index(i, 2), index(i, 1)]);
        indices.append(&mut vec![index(i, 3), index(i, 1), index(i, 2)]);
        // Bottom triangle:
        indices.append(&mut vec![index(i, 4), index(i, 5), 0]);
        // Top triangle:
        indices.append(&mut vec![index(i, 6), 1, index(i, 7)]);
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
