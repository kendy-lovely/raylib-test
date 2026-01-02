use raylib::{color::Color, prelude::*};
use std::{ops::Add};

#[derive(Copy, Clone, PartialEq)] 
pub struct BallEnt {
    pub position: Vector2,
    pub direction: Vector2,
    pub speed: f32,
    pub radius: f32,
    pub color: Color
}

#[derive(Copy, Clone)] 
pub struct RectanglePro {
    pub rect: ffi::Rectangle,
    pub origin: ffi::Vector2,
    pub rotation: f32,
    pub color: Color
}

impl RectanglePro {
    pub fn check_collision_circle_recpro(&self, position: Vector2, radius: f32) -> bool {
        let mut corners: Vec<Vector2> = vec![
            Vector2 { x: -self.origin.x, y: -self.origin.y },                                     // Top-left relative
            Vector2 { x: self.rect.width - self.origin.x, y: -self.origin.y },                    // Top-right relative
            Vector2 { x: self.rect.width - self.origin.x, y: self.rect.height - self.origin.y },  // Bottom-right relative
            Vector2 { x: -self.origin.x, y: self.rect.height - self.origin.y }                    // Bottom-left relative
        ];
        let actual_origin = Vector2 { x: self.rect.x, y: self.rect.y };
        corners.iter_mut().for_each(|corner| *corner = corner.rotated(self.rotation.to_radians()).add(actual_origin));

        let sides: Vec<(Vector2, Vector2)> = corners
            .windows(2)
            .map(|c| (c[0], c[1]))
            .into_iter()
            .chain([(corners[3], corners[0])].into_iter())
            .collect(); 
        
        sides
            .iter()
            .any(|&point| {
                let line_vec: Vector2 = point.1 - point.0;
                let line_vec_sqr: f32 = line_vec.dot(line_vec);
                if line_vec_sqr == 0.0 { check_collision_point_circle(point.0, position, radius) } else {
                    let t = ((position - point.0).dot(line_vec)/line_vec_sqr).clamp(0.0, 1.0);
                    let closest: Vector2 = point.0 + line_vec.scale_by(t);
                    check_collision_point_circle(closest, position, radius)
                }
            })
    }
}