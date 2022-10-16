use std::fmt::Debug;

use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use rand::Rng;

#[derive(Debug)]
pub struct HeightMap {
    pub size: usize,
    pub map: Vec<Vec<f32>>,
}

pub fn generate_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut std_mats: ResMut<Assets<StandardMaterial>>,
) {
    let height_map = gen(10, 100., 100., 1.);
    commands.spawn_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(height_map)),
        transform: Transform::default(),
        material: std_mats.add(Color::GREEN.into()),
        ..default()
    });
}

pub fn gen(n: usize, corners: f32, amplitude: f32, similarity: f32) -> HeightMap {
    let mut map = HeightMap::new(n, corners);
    (0..n).for_each(|i| {
        map.diamond_step(i, amplitude, similarity);
        map.square_step(i, amplitude, similarity);
    });
    map
}

/// h: Self-Similarity parameter. Between 0 and 1.
fn standard_deviation(level: usize, similarity: f32) -> f32 {
    2_f32.powf(-1. * level as f32 * similarity)
}

impl HeightMap {
    fn new(n: usize, corners: f32) -> Self {
        let size = 2_usize.pow(n as u32) + 1;
        let map = (0..size)
            .map(|y| {
                let edge_y = y == 0 || y == size - 1;
                (0..size)
                    .map(|x| {
                        let edge_x = x == 0 || x == size - 1;
                        if edge_x && edge_y {
                            corners
                        } else {
                            0.
                        }
                    })
                    .collect()
            })
            .collect();
        HeightMap { size, map }
    }
    fn diamond_step(&mut self, i: usize, amplitude: f32, similarity: f32) {
        let step = (self.size - 1) / (2_usize.pow(i as u32));
        (0..self.size - 1).step_by(step).for_each(|y| {
            let next_y = (y + step).rem_euclid(self.size);
            (0..self.size - 1).step_by(step).for_each(|x| {
                let next_x = (x + step).rem_euclid(self.size);
                let upper_left = *self.map.get(y).unwrap().get(x).unwrap();
                let lower_left = *self.map.get(next_y).unwrap().get(x).unwrap();
                let upper_right = *self.map.get(y).unwrap().get(next_x).unwrap();
                let lower_right = *self.map.get(next_y).unwrap().get(next_x).unwrap();
                let center = (upper_left + upper_right + lower_left + lower_right) / 4.
                    + amplitude
                        * rand::thread_rng().gen_range(-1.0..1.0)
                        * standard_deviation(i, similarity);
                *self
                    .map
                    .get_mut(x + step / 2)
                    .unwrap()
                    .get_mut(y + step / 2)
                    .unwrap() = center;
            });
        });
    }
    fn square_step(&mut self, i: usize, amplitude: f32, similarity: f32) {
        let step = (self.size - 1) / (2_usize.pow(i as u32));
        let half_step = step / 2;
        (0..self.size).step_by(half_step).for_each(|y| {
            let y_prev = (y as i32 - half_step as i32).rem_euclid(self.size as i32) as usize;
            let y_next = (y + half_step).rem_euclid(self.size);
            let skip = if (y / half_step) % 2 == 0 {
                half_step
            } else {
                0
            };
            (0..self.size).skip(skip).step_by(step).for_each(|x| {
                let x_prev = (x as i32 - half_step as i32).rem_euclid(self.size as i32) as usize;
                let x_next = (x + half_step).rem_euclid(self.size);
                let left = *self.map.get(y).unwrap().get(x_prev).unwrap();
                let right = *self.map.get(y).unwrap().get(x_next).unwrap();
                let top = *self.map.get(y_prev).unwrap().get(x).unwrap();
                let bottom = *self.map.get(y_next).unwrap().get(x).unwrap();
                let center = (left + right + top + bottom) / 4.
                    + amplitude
                        * rand::thread_rng().gen_range(-1.0..1.0)
                        * standard_deviation(i, similarity);
                *self.map.get_mut(x).unwrap().get_mut(y).unwrap() = center;
            });
        });
    }
    /// Face points must be same order as triangle indices, otherwise normal will end up
    /// pointing the exact opposite way.
    fn normal(&self, face: [(i32, i32); 3]) -> Vec3 {
        let point_a = self.point(face[0].0, face[0].1);
        let point_b = self.point(face[1].0, face[1].1);
        let point_c = self.point(face[2].0, face[2].1);
        (point_a - point_b).cross(point_a - point_c).normalize()
    }
    fn point(&self, x: i32, z: i32) -> Vec3 {
        Vec3::new(
            x as f32,
            *self
                .map
                .get(z.rem_euclid(self.size as i32) as usize)
                .unwrap()
                .get(x.rem_euclid(self.size as i32) as usize)
                .unwrap(),
            z as f32,
        )
    }
}

/// Counter-Clockwise mesh.
impl From<HeightMap> for Mesh {
    fn from(hm: HeightMap) -> Self {
        let mut indices: Vec<u32> = Vec::new();
        hm.map
            .iter()
            .enumerate()
            .flat_map(|(z, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, height)| (x, height, z))
            })
            .for_each(|(x, _, z)| {
                let index = x + z * hm.size;
                if x > 0 && z > 0 {
                    indices.append(&mut vec![
                        (index) as u32,
                        (index - hm.size) as u32,
                        (index - 1) as u32,
                    ]);
                }
                if x < hm.size - 1 && z < hm.size - 1 {
                    indices.append(&mut vec![
                        (index) as u32,
                        (index + hm.size) as u32,
                        (index + 1) as u32,
                    ]);
                }
            });
        let size = hm.size;
        let vertices = (0..size)
            .flat_map(|z| (0..size).map(move |x| (x as i32, z as i32)))
            .map(|(x, z)| {
                let position: [f32; 3] = hm.point(x, z).to_array();
                let adjacent_face_normals = [
                    hm.normal([(x, z), (x, z + 1), (x + 1, z)]),
                    hm.normal([(x, z), (x - 1, z + 1), (x, z + 1)]),
                    hm.normal([(x, z), (x - 1, z), (x - 1, z + 1)]),
                    hm.normal([(x, z), (x, z - 1), (x - 1, z)]),
                    hm.normal([(x, z), (x + 1, z - 1), (x, z - 1)]),
                    hm.normal([(x, z), (x + 1, z), (x + 1, z - 1)]),
                ];
                let normal = (adjacent_face_normals.iter().sum::<Vec3>()
                    / adjacent_face_normals.len() as f32)
                    .normalize()
                    .to_array();
                // let normal = [0., 1., 0.];
                let uv = [1., 1.];
                (position, normal, uv)
            })
            .collect::<Vec<_>>();

        let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}
