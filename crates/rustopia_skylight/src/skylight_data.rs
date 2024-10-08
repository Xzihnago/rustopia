use bevy::prelude::*;

pub struct SkylightData {
    pub axis: Vec3,
    pub position: Vec3,
    pub brightness: f32,
    pub illuminance: f32,
    pub color: Color,
}
