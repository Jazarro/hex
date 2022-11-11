use bevy::prelude::Color;
use serde::{Deserialize, Serialize};
use splines::{Interpolation, Key, Spline};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct WorldConfig {
    pub day_night_duration_seconds: f32,
    pub sun_midnight: SunState,
    pub sun_dawn: SunState,
    pub sun_noon: SunState,
    pub sun_dusk: SunState,
}

impl WorldConfig {
    pub fn illuminance(&self) -> Spline<f32, f32> {
        let keys = vec![
            Key::new(0.00, self.sun_midnight.illuminance, Interpolation::Linear),
            Key::new(0.25, self.sun_dawn.illuminance, Interpolation::Linear),
            Key::new(0.50, self.sun_noon.illuminance, Interpolation::Linear),
            Key::new(0.75, self.sun_dusk.illuminance, Interpolation::Linear),
            Key::new(1.00, self.sun_midnight.illuminance, Interpolation::Linear),
        ];
        Spline::from_vec(keys)
    }
    pub fn redness(&self) -> Spline<f32, f32> {
        let keys = vec![
            Key::new(0.00, self.sun_midnight.color.r(), Interpolation::Linear),
            Key::new(0.25, self.sun_dawn.color.r(), Interpolation::Linear),
            Key::new(0.50, self.sun_noon.color.r(), Interpolation::Linear),
            Key::new(0.75, self.sun_dusk.color.r(), Interpolation::Linear),
            Key::new(1.00, self.sun_midnight.color.r(), Interpolation::Linear),
        ];
        Spline::from_vec(keys)
    }
    pub fn greenness(&self) -> Spline<f32, f32> {
        let keys = vec![
            Key::new(0.00, self.sun_midnight.color.g(), Interpolation::Linear),
            Key::new(0.25, self.sun_dawn.color.g(), Interpolation::Linear),
            Key::new(0.50, self.sun_noon.color.g(), Interpolation::Linear),
            Key::new(0.75, self.sun_dusk.color.g(), Interpolation::Linear),
            Key::new(1.00, self.sun_midnight.color.g(), Interpolation::Linear),
        ];
        Spline::from_vec(keys)
    }
    pub fn blueness(&self) -> Spline<f32, f32> {
        let keys = vec![
            Key::new(0.00, self.sun_midnight.color.b(), Interpolation::Linear),
            Key::new(0.25, self.sun_dawn.color.b(), Interpolation::Linear),
            Key::new(0.50, self.sun_noon.color.b(), Interpolation::Linear),
            Key::new(0.75, self.sun_dusk.color.b(), Interpolation::Linear),
            Key::new(1.00, self.sun_midnight.color.b(), Interpolation::Linear),
        ];
        Spline::from_vec(keys)
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SunState {
    pub illuminance: f32,
    pub color: Color,
}
