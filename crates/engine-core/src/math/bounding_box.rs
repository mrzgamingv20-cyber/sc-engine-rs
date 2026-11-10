use super::vector3::Vector3;
use super::matrix::Matrix;
use super::bounding_sphere::BoundingSphere;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BoundingBox {
    pub min: Vector3,
    pub max: Vector3,
}

impl BoundingBox {
    #[inline]
    pub fn new(min: Vector3, max: Vector3) -> Self {
        BoundingBox { min, max }
    }

    #[inline]
    pub fn from_coords(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> Self {
        BoundingBox::new(Vector3::new(x1, y1, z1), Vector3::new(x2, y2, z2))
    }

    pub fn from_points(points: &[Vector3]) -> Option<BoundingBox> {
        if points.is_empty() {
            return None;
        }
        let mut min = Vector3::splat(f32::INFINITY);
        let mut max = Vector3::splat(f32::NEG_INFINITY);
        for p in points {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            min.z = min.z.min(p.z);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
            max.z = max.z.max(p.z);
        }
        Some(BoundingBox::new(min, max))
    }

    pub fn corners(&self) -> [Vector3; 8] {
        [
            Vector3::new(self.min.x, self.min.y, self.min.z),
            Vector3::new(self.max.x, self.min.y, self.min.z),
            Vector3::new(self.min.x, self.max.y, self.min.z),
            Vector3::new(self.max.x, self.max.y, self.min.z),
            Vector3::new(self.min.x, self.min.y, self.max.z),
            Vector3::new(self.max.x, self.min.y, self.max.z),
            Vector3::new(self.min.x, self.max.y, self.max.z),
            Vector3::new(self.max.x, self.max.y, self.max.z),
        ]
    }

    #[inline]
    pub fn center(&self) -> Vector3 {
        Vector3::new(
            0.5 * (self.min.x + self.max.x),
            0.5 * (self.min.y + self.max.y),
            0.5 * (self.min.z + self.max.z),
        )
    }

    #[inline]
    pub fn size(&self) -> Vector3 {
        self.max - self.min
    }

    pub fn volume(&self) -> f32 {
        let s = self.size();
        s.x * s.y * s.z
    }

    #[inline]
    pub fn contains(&self, p: Vector3) -> bool {
        p.x >= self.min.x && p.x <= self.max.x
            && p.y >= self.min.y && p.y <= self.max.y
            && p.z >= self.min.z && p.z <= self.max.z
    }

    #[inline]
    pub fn intersects_box(&self, other: &BoundingBox) -> bool {
        other.max.x >= self.min.x
            && other.min.x <= self.max.x
            && other.max.y >= self.min.y
            && other.min.y <= self.max.y
            && other.max.z >= self.min.z
            && other.min.z <= self.max.z
    }

    pub fn intersects_sphere(&self, sphere: &BoundingSphere) -> bool {
        let c = &sphere.center;
        let r = sphere.radius;
        if c.x - self.min.x > r && c.y - self.min.y > r && c.z - self.min.z > r
            && self.max.x - c.x > r && self.max.y - c.y > r && self.max.z - c.z > r {
            return true;
        }
        let mut num = 0.0f32;
        if c.x - self.min.x <= r {
            let d = c.x - self.min.x;
            num += d * d;
        } else if self.max.x - c.x <= r {
            let d = c.x - self.max.x;
            num += d * d;
        }
        if c.y - self.min.y <= r {
            let d = c.y - self.min.y;
            num += d * d;
        } else if self.max.y - c.y <= r {
            let d = c.y - self.max.y;
            num += d * d;
        }
        if c.z - self.min.z <= r {
            let d = c.z - self.min.z;
            num += d * d;
        } else if self.max.z - c.z <= r {
            let d = c.z - self.max.z;
            num += d * d;
        }
        num <= r * r
    }

    pub fn intersection(b1: &BoundingBox, b2: &BoundingBox) -> BoundingBox {
        let min = Vector3::max_vec(b1.min, b2.min);
        let max = Vector3::min_vec(b1.max, b2.max);
        if max.x > min.x && max.y > min.y && max.z > min.z {
            BoundingBox::new(min, max)
        } else {
            BoundingBox::default()
        }
    }

    pub fn union(b1: &BoundingBox, b2: &BoundingBox) -> BoundingBox {
        BoundingBox::new(
            Vector3::min_vec(b1.min, b2.min),
            Vector3::max_vec(b1.max, b2.max),
        )
    }

    pub fn union_point(b: &BoundingBox, p: Vector3) -> BoundingBox {
        BoundingBox::new(
            Vector3::min_vec(b.min, p),
            Vector3::max_vec(b.max, p),
        )
    }

    pub fn distance(b: &BoundingBox, p: Vector3) -> f32 {
        let dx = (b.min.x - p.x).max(0.0).max(p.x - b.max.x);
        let dy = (b.min.y - p.y).max(0.0).max(p.y - b.max.y);
        let dz = (b.min.z - p.z).max(0.0).max(p.z - b.max.z);
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn transform(b: &BoundingBox, m: &Matrix) -> BoundingBox {
        let corners = b.corners();
        let mut transformed = [Vector3::ZERO; 8];
        for (i, c) in corners.iter().enumerate() {
            transformed[i] = Vector3::transform(*c, m);
        }
        BoundingBox::from_points(&transformed).unwrap_or_default()
    }
}

impl std::fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.min, self.max)
    }
}
