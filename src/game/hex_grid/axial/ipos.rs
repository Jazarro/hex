/// A hexagonal coordinate in the flat-topped axial coordinate system.
/// For more information, see https://www.redblobgames.com/grids/hexagons/
pub struct IPos {
    pub q: i32,
    pub r: i32,
    /// The height. This is unchanged from the regular xyz coordinate system.
    pub z: i32,
}
