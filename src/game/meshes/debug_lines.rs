use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::mesh::{Indices, MeshVertexBufferLayout, PrimitiveTopology};
use bevy::render::render_resource::{
    AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
};

use crate::animate_simple::{RotAxis, RotateTag};
use crate::assets::config::config_debug::DebugConfig;
use crate::assets::config::config_debug::OriginLinesDisplay;
use crate::{default, Color, MaterialMeshBundle, Mesh, Transform, Vec3};

pub fn apply_debug_lines(
    mut commands: Commands,
    config: Res<DebugConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut line_mats: ResMut<Assets<LineMaterial>>,
    mut std_mats: ResMut<Assets<StandardMaterial>>,
) {
    if matches!(config.origin_lines_display, OriginLinesDisplay::Disabled) {
        return;
    }
    let min = if matches!(config.origin_lines_display, OriginLinesDisplay::Both) {
        -config.origin_lines_length
    } else {
        0.
    };
    let max = config.origin_lines_length;
    // x-axis: Blue
    commands.spawn_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(LineList {
            lines: vec![(Vec3::new(min, 0., 0.), Vec3::new(max, 0., 0.))],
        })),
        transform: Transform::default(),
        material: line_mats.add(LineMaterial { color: Color::BLUE }),
        ..default()
    });
    commands.spawn_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(Cone::default())),
        transform: Transform::from_xyz(max, 0., 0.)
            .with_scale(Vec3::splat(config.origin_lines_cone_scale))
            .with_rotation(Quat::from_rotation_z(std::f32::consts::TAU * -0.25)),
        material: std_mats.add(Color::BLUE.into()),
        ..default()
    });
    if matches!(config.origin_lines_display, OriginLinesDisplay::Both) {
        commands.spawn_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(Cone::default())),
            transform: Transform::from_xyz(min, 0., 0.)
                .with_scale(Vec3::splat(config.origin_lines_cone_scale))
                .with_rotation(Quat::from_rotation_z(std::f32::consts::TAU * 0.25)),
            material: std_mats.add(Color::BLUE.into()),
            ..default()
        });
    }
    // y-axis: Green
    commands.spawn_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(LineList {
            lines: vec![(Vec3::new(0., min, 0.), Vec3::new(0., max, 0.))],
        })),
        transform: Transform::default(),
        material: line_mats.add(LineMaterial {
            color: Color::GREEN,
        }),
        ..default()
    });
    commands.spawn_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(Cone::default())),
        transform: Transform::from_xyz(0., max, 0.)
            .with_scale(Vec3::splat(config.origin_lines_cone_scale)),
        material: std_mats.add(Color::GREEN.into()),
        ..default()
    });
    if matches!(config.origin_lines_display, OriginLinesDisplay::Both) {
        commands.spawn_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(Cone::default())),
            transform: Transform::from_xyz(0., min, 0.)
                .with_scale(Vec3::splat(config.origin_lines_cone_scale))
                .with_rotation(Quat::from_rotation_x(std::f32::consts::TAU * 0.5)),
            material: std_mats.add(Color::GREEN.into()),
            ..default()
        });
    }
    // z-axis: Red
    commands.spawn_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(LineList {
            lines: vec![(Vec3::new(0., 0., min), Vec3::new(0., 0., max))],
        })),
        transform: Transform::default(),
        material: line_mats.add(LineMaterial { color: Color::RED }),
        ..default()
    });
    commands.spawn_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(Cone::default())),
        transform: Transform::from_xyz(0., 0., max)
            .with_scale(Vec3::splat(config.origin_lines_cone_scale))
            .with_rotation(Quat::from_rotation_x(std::f32::consts::TAU * 0.25)),
        material: std_mats.add(Color::RED.into()),
        ..default()
    });
    if matches!(config.origin_lines_display, OriginLinesDisplay::Both) {
        commands.spawn_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(Cone::default())),
            transform: Transform::from_xyz(0., 0., min)
                .with_scale(Vec3::splat(config.origin_lines_cone_scale))
                .with_rotation(Quat::from_rotation_x(std::f32::consts::TAU * -0.25)),
            material: std_mats.add(Color::RED.into()),
            ..default()
        });
    }
}

pub fn spawn_cone(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut std_mats: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(Cone::default())),
            transform: Transform::default().with_scale(Vec3::splat(20.)),
            material: std_mats.add(Color::GREEN.into()),
            ..default()
        })
        .insert(RotateTag {
            timer: Timer::from_seconds(4., true),
            axis: RotAxis::Chain,
        });
}

/// A list of lines with a start and end position
#[derive(Debug, Clone)]
pub struct LineList {
    pub lines: Vec<(Vec3, Vec3)>,
}

impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        let mut vertices = vec![];
        let mut normals = vec![];
        for (start, end) in line.lines {
            vertices.push(start.to_array());
            vertices.push(end.to_array());
            normals.push(Vec3::ZERO.to_array());
            normals.push(Vec3::ZERO.to_array());
        }

        // This tells wgpu that the positions are list of lines
        // where every pair is a start and end point
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        // Normals are currently required by bevy, but they aren't used by the [`LineMaterial`]
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh
    }
}

#[derive(Default, AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "050ce6ac-080a-4d8c-b6b5-b5bab7560d8f"]
pub struct LineMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        "default/shaders/line_material.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // This is the important part to tell bevy to render this material as a line between vertices
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Cone {
    pub height: f32,
    pub radius: f32,
    pub nr_facets: u32,
}

impl Default for Cone {
    fn default() -> Self {
        Cone {
            height: 0.5,
            radius: 0.125,
            nr_facets: 16,
        }
    }
}

impl From<Cone> for Mesh {
    fn from(cone: Cone) -> Self {
        let vertex_top = (Vec3::new(0., cone.height, 0.), [0.0, 1.0, 0.0], [1.0, 1.0]);
        let vertex_bottom = (Vec3::new(0., 0., 0.), [0.0, -1.0, 0.0], [1.0, 1.0]);
        let mut vertices_base = vec![];
        let mut vertices_hood = vec![];
        let circle_points = (0..cone.nr_facets)
            .map(|i| {
                let angle = (std::f32::consts::TAU / cone.nr_facets as f32) * i as f32;
                Vec3::new(angle.cos() * cone.radius, 0., angle.sin() * cone.radius)
            })
            .collect::<Vec<Vec3>>();

        for i in 0..cone.nr_facets as i32 {
            let top = vertex_top.0;
            let point_a = *circle_points
                .get((i - 1).rem_euclid(cone.nr_facets as i32) as usize)
                .unwrap();
            let point_b = *circle_points.get(i as usize).unwrap();
            let point_c = *circle_points
                .get((i + 1).rem_euclid(cone.nr_facets as i32) as usize)
                .unwrap();
            let left_face_normal = (top - point_a).cross(point_b - point_a).normalize();
            let right_face_normal = (top - point_b).cross(point_c - point_b).normalize();
            let avg_normal = ((left_face_normal + right_face_normal) / 2.).normalize();
            vertices_base.push((point_b, [0.0, -1.0, 0.0], [1.0, 1.0]));
            vertices_hood.push((point_b, avg_normal.to_array(), [1.0, 1.0]));
        }

        let mut vertices = Vec::new();
        vertices.push(vertex_top);
        vertices.push(vertex_bottom);
        vertices.append(&mut vertices_base);
        vertices.append(&mut vertices_hood);

        let mut indices = Vec::new();
        for i in 0..cone.nr_facets {
            indices.append(&mut vec![(i + 1) % cone.nr_facets + 2, 1, i + 2]);
        }
        for i in 0..cone.nr_facets {
            indices.append(&mut vec![
                i + 2 + cone.nr_facets,
                0,
                (i + 1) % cone.nr_facets + 2 + cone.nr_facets,
            ]);
        }

        let positions: Vec<_> = vertices.iter().map(|(p, _, _)| p.to_array()).collect();
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
