#[derive(Copy, Clone, Debug)]
pub enum BiomeType {
    BorealForest,
    Desert,
    Forest,
    Grassland,
    Ice,
    Jungle,
    Savanna,
    Swamp,
    Tundra,
}

/// TODO: Add elevation as a parameter.
pub fn generate_biomes(humidity_noise: Vec<f64>, temperature_noise: Vec<f64>) -> Vec<BiomeType> {
    let mut biomes = Vec::new();

    for i in 0..humidity_noise.len() {
        let humidity = humidity_noise[i];
        let temperature = temperature_noise[i];

        let biome = match (humidity, temperature) {
            (humidity, temperature) if humidity < 0.33 && temperature < 0.33 => BiomeType::Ice,
            (humidity, temperature) if humidity < 0.33 && temperature < 0.66 => BiomeType::Tundra,
            (humidity, temperature) if humidity < 0.33 && temperature < 1.0 => {
                BiomeType::BorealForest
            }
            (humidity, temperature) if humidity < 0.66 && temperature < 0.33 => BiomeType::Desert,
            (humidity, temperature) if humidity < 0.66 && temperature < 0.66 => {
                BiomeType::Grassland
            }
            (humidity, temperature) if humidity < 0.66 && temperature < 1.0 => BiomeType::Savanna,
            (humidity, temperature) if humidity < 1.0 && temperature < 0.33 => BiomeType::Swamp,
            (humidity, temperature) if humidity < 1.0 && temperature < 0.66 => BiomeType::Jungle,
            (humidity, temperature) if humidity < 1.0 && temperature < 1.0 => BiomeType::Forest,
            _ => BiomeType::Grassland,
        };

        biomes.push(biome);
    }

    biomes
}
