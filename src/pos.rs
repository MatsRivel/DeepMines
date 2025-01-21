use super::*;

#[derive(Debug, Component, Clone, Copy, Eq, PartialEq)]
pub struct GridPosition {
    x: usize,
    y: usize,
}
impl GridPosition {
    fn min_pos() -> f32 {
        ((N_TILES - 1) as f32 * STEP_SIZE) / 2f32
    }
    pub fn new(x: usize, y: usize) -> Self {
        let x = ((N_TILES - 1) as usize).min(x);
        let y = ((N_TILES - 1) as usize).min(y);
        Self { x, y }
    }
    fn to_usize(v: f32) -> usize {
        let v = v + Self::min_pos();
        let v = (v - 1f32) / STEP_SIZE;
        let v = v.round();
        v as usize
    }
    fn to_float(v: usize) -> f32 {
        let v = v as f32 * STEP_SIZE;

        v - Self::min_pos()
    }
    pub fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
    pub fn center() -> GridPosition {
        let v = N_TILES as usize / 2;
        Self::new(v, v)
    }
}
impl From<Vec3> for GridPosition {
    fn from(value: Vec3) -> GridPosition {
        let x = GridPosition::to_usize(value.x);
        let y = GridPosition::to_usize(value.y);
        Self::new(x, y)
    }
}
impl From<GridPosition> for Vec3 {
    fn from(value: GridPosition) -> Vec3 {
        let x = GridPosition::to_float(value.x);
        let y = GridPosition::to_float(value.y);
        Vec3::new(x, y, 0.0)
    }
}
