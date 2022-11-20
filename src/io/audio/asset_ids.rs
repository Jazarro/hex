use serde::{Deserialize, Serialize};

use hex_derive::MusicId;
use hex_derive::SfxId;

pub trait SfxId {
    /// The name of the enum.
    fn group_id(&self) -> &'static str;
    /// The enum value.
    fn item_id(&self) -> &'static str;
}

pub trait MusicId {
    /// The name of the enum.
    fn group_id(&self) -> &'static str;
    /// The enum value.
    fn item_id(&self) -> &'static str;
}

#[derive(SfxId, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub enum SfxMonster {
    Idle,
    Aggro,
    Hit,
    Death,
}

#[derive(MusicId, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub enum BackgroundMusic {
    ExistentialHexMan,
    HexagonalCubes,
}
