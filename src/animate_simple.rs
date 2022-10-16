use bevy::math::Quat;
use bevy::prelude::{Component, Query, Res, Time, Timer, Transform};

pub fn rotate(time: Res<Time>, mut query: Query<(&mut Transform, &mut RotateTag)>) {
    for (mut transform, mut tag) in query.iter_mut() {
        tag.timer.tick(time.delta());
        let (s, c) = (tag.axis.angle(tag.timer.percent()) * 0.5).sin_cos();
        let x = if tag.axis.is_x(tag.timer.percent()) {
            s
        } else {
            0.
        };
        let y = if tag.axis.is_y(tag.timer.percent()) {
            s
        } else {
            0.
        };
        let z = if tag.axis.is_z(tag.timer.percent()) {
            s
        } else {
            0.
        };
        transform.rotation = Quat::from_xyzw(x, y, z, c);
    }
}

#[derive(Component)]
pub struct RotateTag {
    pub timer: Timer,
    pub axis: RotAxis,
}

pub enum RotAxis {
    X,
    Y,
    Z,
    Chain,
}

impl RotAxis {
    fn angle(&self, percent: f32) -> f32 {
        (if matches!(self, RotAxis::Chain) {
            percent * 3. - (percent * 3.).floor()
        } else {
            percent
        }) * std::f32::consts::TAU
    }
    fn is_x(&self, percent: f32) -> bool {
        matches!(self, RotAxis::X) || (matches!(self, RotAxis::Chain) && percent * 3. < 1.)
    }
    fn is_y(&self, percent: f32) -> bool {
        matches!(self, RotAxis::Y)
            || (matches!(self, RotAxis::Chain) && percent * 3. >= 1. && percent * 3. < 2.)
    }
    fn is_z(&self, percent: f32) -> bool {
        matches!(self, RotAxis::Z) || (matches!(self, RotAxis::Chain) && percent * 3. >= 2.)
    }
}
