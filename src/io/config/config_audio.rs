use bevy::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Resource, Debug, Default, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AudioConfig {}
