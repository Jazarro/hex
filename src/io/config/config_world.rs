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

    /// As long as chunks within this radius of the player are rendered, no loading / unloading will happen.
    /// This value should equal at most the render_distance_max.
    pub render_distance_min: u32,
    /// When a load / unload is triggered, chunks within this radius will be rendered around the player.
    /// If this is set to zero, only the chunk that the player is on will be rendered.
    ///
    /// Note that one additional ring of chunks will be loaded into memory, but not rendered.
    /// This is primarily to make sure sides of chunks don't end up in the mesh unnecessarily.
    pub render_distance_max: u32,
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
