use crate::types::Vec3;

pub struct Color(pub u8, pub u8, pub u8);

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        let v = v.sqrt() * 255.99;
        let (r, g, b) = (v.x(), v.y(), v.z());

        Self(r as u8, g as u8, b as u8)
    }
}
