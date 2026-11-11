//! Render viewport and projection helpers.

use crate::math::{Matrix, Rectangle, Vector3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub min_depth: f32,
    pub max_depth: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Viewport { x: 0, y: 0, width: 0, height: 0, min_depth: 0.0, max_depth: 1.0 }
    }
}

impl Viewport {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Viewport { x, y, width, height, min_depth: 0.0, max_depth: 1.0 }
    }

    pub fn with_depth(x: i32, y: i32, width: i32, height: i32, min_depth: f32, max_depth: f32) -> Self {
        Viewport { x, y, width, height, min_depth, max_depth }
    }

    #[inline]
    pub fn rectangle(&self) -> Rectangle {
        Rectangle::new(self.x, self.y, self.width, self.height)
    }

    #[inline]
    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    /// Project a world-space point into screen space.
    pub fn project(&self, source: Vector3, world_view_projection: &Matrix) -> Vector3 {
        let mut result = Vector3::transform(source, world_view_projection);
        let w = source.x * world_view_projection.m14
            + source.y * world_view_projection.m24
            + source.z * world_view_projection.m34
            + world_view_projection.m44;
        result /= w;
        result.x = (result.x + 1.0) * 0.5 * self.width as f32 + self.x as f32;
        result.y = (0.0 - result.y + 1.0) * 0.5 * self.height as f32 + self.y as f32;
        result.z = result.z * (self.max_depth - self.min_depth) + self.min_depth;
        result
    }

    pub fn project_full(&self, source: Vector3, projection: &Matrix, view: &Matrix, world: &Matrix) -> Vector3 {
        self.project(source, &(*world * *view * *projection))
    }

    /// Convert a screen-space point back into world space.
    pub fn unproject(&self, source: Vector3, world_view_projection: &Matrix) -> Vector3 {
        let m = Matrix::invert(world_view_projection);
        let mut s = source;
        s.x = (s.x - self.x as f32) / self.width as f32 * 2.0 - 1.0;
        s.y = 0.0 - ((s.y - self.y as f32) / self.height as f32 * 2.0 - 1.0);
        s.z = (s.z - self.min_depth) / (self.max_depth - self.min_depth);
        let w = s.x * m.m14 + s.y * m.m24 + s.z * m.m34 + m.m44;
        Vector3::transform(s, &m) / w
    }

    pub fn unproject_full(&self, source: Vector3, projection: &Matrix, view: &Matrix, world: &Matrix) -> Vector3 {
        self.unproject(source, &(*world * *view * *projection))
    }
}