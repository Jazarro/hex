use bevy::math::DVec2;
use bevy::prelude::*;
use noise::{NoiseFn, OpenSimplex};

const SEED: u32 = 123456789;

pub enum NoiseLayer {
    Elevation,
    Humidity,
    Temperature,
}

/// `random_x_values` must be the same length or greater than the length of `octaves`.
/// `random_y_values` must be the same length or greater than the length of `octaves`.
pub struct NoiseProfile {
    pub scale: f64,
    pub octaves: u32,
    pub persistence: f64,
    pub lacunarity: f64,
    pub offset: Vec2,
    pub threshold: f64,
    pub random_q_values: Vec<f64>,
    pub random_r_values: Vec<f64>,
}

impl NoiseProfile {
    pub fn new(
        scale: f64,
        octaves: u32,
        persistence: f64,
        lacunarity: f64,
        offset: Vec2,
        threshold: f64,
        random_q_values: Vec<f64>,
        random_r_values: Vec<f64>,
    ) -> Self {
        NoiseProfile {
            scale,
            octaves,
            persistence,
            lacunarity,
            offset,
            threshold,
            random_q_values,
            random_r_values,
        }
    }
}

pub fn get_noise_profile(noise_layer: NoiseLayer) -> NoiseProfile {
    match noise_layer {
        NoiseLayer::Elevation => NoiseProfile::new(
            183.0,
            5,
            0.411,
            2.61,
            Vec2::new(0.0, 0.0),
            0.5,
            vec![
                -60194.0, 83225.0, -26479.0, -66750.0, 81086.0, 82226.0, 42251.0, -49274.0, 4072.0,
                69614.0,
            ],
            vec![
                -18821.0, 75048.0, 98239.0, -34072.0, 86031.0, 42299.0, -54339.0, 98982.0, 88495.0,
                -55394.0,
            ],
        ),
        NoiseLayer::Humidity => NoiseProfile::new(
            150.0,
            3,
            0.161,
            2.42,
            Vec2::new(0.0, 0.0),
            0.5,
            vec![
                89325.0, -61393.0, 82073.0, -10297.0, -23954.0, -51626.0, 84690.0, -59074.0,
                4053.0, 86540.0,
            ],
            vec![
                -22438.0, -87216.0, 69093.0, 3199.0, -91910.0, 35240.0, -62759.0, 65496.0, 65302.0,
                -48971.0,
            ],
        ),
        NoiseLayer::Temperature => NoiseProfile::new(
            227.0,
            3,
            0.13,
            4.09,
            Vec2::new(0.0, 0.0),
            0.5,
            vec![
                97568.0, -16994.0, 23570.0, 317.0, 64823.0, 48324.0, -95148.0, 29953.0, 65217.0,
                -53326.0,
            ],
            vec![
                -66347.0, 92566.0, -45926.0, -31324.0, 2510.0, -61129.0, 50871.0, -94602.0,
                -18903.0, 10752.0,
            ],
        ),
    }
}

pub fn generate_noise(position: DVec2, bounds: IVec2, profile: NoiseProfile) -> Vec<f64> {
    let mut noise: Vec<f64> = Vec::new();
    let open_simplex = OpenSimplex::new(SEED);

    let mut scale = profile.scale;
    let octaves = profile.octaves;
    let persistence = profile.persistence;
    let lacunarity = profile.lacunarity;
    let offset = profile.offset;
    let threshold = profile.threshold;
    let random_q_values = profile.random_q_values;
    let random_r_values = profile.random_r_values;

    let mut octave_offsets: Vec<Vec2> = Vec::new();
    for i in 0..octaves {
        let q_offset = random_q_values[i as usize] + offset.x as f64;
        let r_offset = random_r_values[i as usize] + offset.y as f64;
        let offset = Vec2::new(q_offset as f32, r_offset as f32);
        octave_offsets.push(offset);
    }

    if scale <= 0.0 {
        scale = 0.0001;
    }

    let half_width = bounds.x as f64 / 2.0;
    let half_height = bounds.y as f64 / 2.0;
    let mut row_offset = 0.0;

    for y in 0..bounds.y {
        for x in 0..bounds.x {
            let mut amplitude: f64 = 1.0;
            let mut frequency: f64 = 1.0;
            let mut noise_value: f64 = 0.0;

            for i in 0..octaves {
                let mut q_sample = x as f64 + row_offset as f64 + position.x; // * CHUNK_WIDTH ???
                let mut r_sample = y as f64 + position.y; // * CHUNK_HEIGHT ???
                q_sample = (q_sample - half_width) / scale * frequency
                    + octave_offsets[i as usize].x as f64;
                r_sample = (r_sample - half_height) / scale * frequency
                    + octave_offsets[i as usize].y as f64;

                let noise_sample = open_simplex.get([q_sample, r_sample, 0.0]);

                noise_value += noise_sample * amplitude;
                amplitude *= persistence;
                frequency *= lacunarity;
            }

            noise.push(noise_value);
        }
        row_offset += 0.5;
    }

    noise
}

// References
// Random Number Generator
// 1. https://www.calculator.net/random-number-generator.html?clower=-100000&cupper=100000&cnums=10&cdup=n&csort=n&cnumt=i&cprec=50&ctype=2&s=84136.2214&submit1=Generate#comprehensive
