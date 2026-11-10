use super::bounding_rectangle::BoundingRectangle;
use super::math_utils::max_f32;
use super::vector2::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Ray2 {
    pub position: Vector2,
    pub direction: Vector2,
}

impl Ray2 {
    #[inline]
    pub fn new(position: Vector2, direction: Vector2) -> Self {
        Ray2 { position, direction }
    }

    pub fn intersection(&self, rectangle: &BoundingRectangle) -> Option<f32> {
        let mut num = 0.0f32;

        // X slab
        if self.direction.x == 0.0 {
            if self.position.x < rectangle.min.x || self.position.x > rectangle.max.x {
                return None;
            }
        } else {
            let num2 = 1.0 / self.direction.x;
            let mut num3 = (rectangle.min.x - self.position.x) * num2;
            let mut num4 = (rectangle.max.x - self.position.x) * num2;
            if num3 > num4 {
                std::mem::swap(&mut num3, &mut num4);
            }
            num = max_f32(num3, num);
            if num > num4 {
                return None;
            }
        }

        // Y slab
        if self.direction.y == 0.0 {
            if self.position.y < rectangle.min.y || self.position.y > rectangle.max.y {
                return None;
            }
        } else {
            let num6 = 1.0 / self.direction.y;
            let mut num7 = (rectangle.min.y - self.position.y) * num6;
            let mut num8 = (rectangle.max.y - self.position.y) * num6;
            if num7 > num8 {
                std::mem::swap(&mut num7, &mut num8);
            }
            num = max_f32(num7, num);
            if num > num8 {
                return None;
            }
        }

        Some(num)
    }
}

impl std::fmt::Display for Ray2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.position, self.direction)
    }
}
