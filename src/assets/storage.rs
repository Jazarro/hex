use std::collections::HashMap;

use crate::assets::config::configs::Config;

#[derive(Default)]
pub struct AssetStorage {
    pub map: HashMap<String, Vec<Config>>,
}
