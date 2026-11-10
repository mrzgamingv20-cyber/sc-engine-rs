use super::vector3::Vector3;
use super::matrix::Matrix;
use super::bounding_box::BoundingBox;
use super::bounding_sphere::BoundingSphere;
use super::plane::Plane;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray3 {
    pub position: Vector3,
    pub direction: Vector3,
}

impl Ray3 {
    #[inline]
    pub fn new(position: Vector3, direction: Vector3) -> Self {
        Ray3 { position, direction }
    }

    pub fn intersect_bounding_box(&self, box_: &BoundingBox) -> Option<f32> {
        let p = self.position;
        let d = self.direction;
        if p.x >= box_.min.x && p.x <= box_.max.x
            && p.y >= box_.min.y && p.y <= box_.max.y
            && p.z >= box_.min.z && p.z <= box_.max.z {
            return Some(0.0);
        }
        let mut t = Vector3::splat(-1.0);
        if d.x != 0.0 {
            if p.x < box_.min.x {
                t.x = (box_.min.x - p.x) / d.x;
            } else if p.x > box_.max.x {
                t.x = (box_.max.x - p.x) / d.x;
            }
        }
        if d.y != 0.0 {
            if p.y < box_.min.y {
                t.y = (box_.min.y - p.y) / d.y;
            } else if p.y > box_.max.y {
                t.y = (box_.max.y - p.y) / d.y;
            }
        }
        if d.z != 0.0 {
            if p.z < box_.min.z {
                t.z = (box_.min.z - p.z) / d.z;
            } else if p.z > box_.max.z {
                t.z = (box_.max.z - p.z) / d.z;
            }
        }
        if t.x > t.y && t.x > t.z {
            if t.x < 0.0 { return None; }
            let num = p.z + t.x * d.z;
            if num < box_.min.z || num > box_.max.z { return None; }
            let num = p.y + t.x * d.y;
            if num < box_.min.y || num > box_.max.y { return None; }
            return Some(t.x);
        }
        if t.y > t.x && t.y > t.z {
            if t.y < 0.0 { return None; }
            let num = p.z + t.y * d.z;
            if num < box_.min.z || num > box_.max.z { return None; }
            let num = p.x + t.y * d.x;
            if num < box_.min.x || num > box_.max.x { return None; }
            return Some(t.y);
        }
        if t.z < 0.0 { return None; }
        let num = p.x + t.z * d.x;
        if num < box_.min.x || num > box_.max.x { return None; }
        let num = p.y + t.z * d.y;
        if num < box_.min.y || num > box_.max.y { return None; }
        Some(t.z)
    }

    pub fn intersect_bounding_sphere(&self, sphere: &BoundingSphere) -> Option<f32> {
        let v = sphere.center - self.position;
        let num = v.length_squared();
        let num2 = sphere.radius * sphere.radius;
        if num < num2 {
            return Some(0.0);
        }
        let num3 = Vector3::dot(self.direction, v);
        if num3 < 0.0 {
            return None;
        }
        let num4 = num2 + num3 * num3 - num;
        if num4 < 0.0 {
            return None;
        }
        Some(num3 - num4.sqrt())
    }

    pub fn intersect_plane(&self, plane: &Plane) -> Option<f32> {
        let num = Vector3::dot(self.direction, plane.normal);
        if num == 0.0 {
            return None;
        }
        Some(-(Vector3::dot(self.position, plane.normal) + plane.d) / num)
    }

    #[inline]
    pub fn sample(&self, distance: f32) -> Vector3 {
        self.position + self.direction * distance
    }

    pub fn transform(r: &Ray3, m: &Matrix) -> Ray3 {
        Ray3::new(
            Vector3::transform(r.position, m),
            Vector3::transform_normal(r.direction, m),
        )
    }
}

impl std::fmt::Display for Ray3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.position, self.direction)
    }
}
