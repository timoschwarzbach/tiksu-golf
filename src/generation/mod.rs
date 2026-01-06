use crate::material::ground::Polynomial;

pub mod grasslands;

pub enum PropType {
    Tree,
}

pub struct Prop {
    pub prop_type: PropType,
    pub position: (f32, f32, f32),
    pub seed: u32,
}

#[derive(PartialEq)]
pub enum ZoneType {
    DeadZone,
    Clean,
    Offtrack,
}

pub trait TerrainGenerator {
    fn height_at(&self, x: f32, y: f32) -> f32;
    fn props_in_chunk(&self, offset: (i32, i32)) -> Vec<Prop>;
    fn course_layout(&self) -> Polynomial;
    fn start(&self) -> [f32; 2];
    fn hole(&self) -> [f32; 2];
    fn zone_type_at(&self, x: f32, y: f32) -> ZoneType;
}
