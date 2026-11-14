use super::bounding_box::BoundingBox;
use super::bounding_sphere::BoundingSphere;
use super::matrix::Matrix;
use super::plane::Plane;
use super::ray3::Ray3;
use super::vector3::Vector3;

pub struct BoundingFrustum {
    view_projection: Matrix,
    planes: [Plane; 6],
    corners: [Vector3; 8],
    corners_valid: bool,
}

impl BoundingFrustum {
    pub fn near(&self) -> Plane { self.planes[0] }
    pub fn far(&self) -> Plane { self.planes[1] }
    pub fn left(&self) -> Plane { self.planes[2] }
    pub fn right(&self) -> Plane { self.planes[3] }
    pub fn top(&self) -> Plane { self.planes[4] }
    pub fn bottom(&self) -> Plane { self.planes[5] }

    pub fn matrix(&self) -> Matrix { self.view_projection }

    pub fn set_matrix(&mut self, value: Matrix) {
        self.view_projection = value;

        // Near plane
        self.planes[0].normal.x = -value.m13;
        self.planes[0].normal.y = -value.m23;
        self.planes[0].normal.z = -value.m33;
        self.planes[0].d = -value.m43;

        // Far plane
        self.planes[1].normal.x = -value.m14 + value.m13;
        self.planes[1].normal.y = -value.m24 + value.m23;
        self.planes[1].normal.z = -value.m34 + value.m33;
        self.planes[1].d = -value.m44 + value.m43;

        // Left plane
        self.planes[2].normal.x = -value.m14 - value.m11;
        self.planes[2].normal.y = -value.m24 - value.m21;
        self.planes[2].normal.z = -value.m34 - value.m31;
        self.planes[2].d = -value.m44 - value.m41;

        // Right plane
        self.planes[3].normal.x = -value.m14 + value.m11;
        self.planes[3].normal.y = -value.m24 + value.m21;
        self.planes[3].normal.z = -value.m34 + value.m31;
        self.planes[3].d = -value.m44 + value.m41;

        // Top plane
        self.planes[4].normal.x = -value.m14 + value.m12;
        self.planes[4].normal.y = -value.m24 + value.m22;
        self.planes[4].normal.z = -value.m34 + value.m32;
        self.planes[4].d = -value.m44 + value.m42;

        // Bottom plane
        self.planes[5].normal.x = -value.m14 - value.m12;
        self.planes[5].normal.y = -value.m24 - value.m22;
        self.planes[5].normal.z = -value.m34 - value.m32;
        self.planes[5].d = -value.m44 - value.m42;

        // Normalize planes
        for plane in &mut self.planes {
            let num = plane.normal.length();
            if num > 0.0 {
                plane.normal /= num;
                plane.d /= num;
            }
        }

        self.corners_valid = false;
    }

    pub fn new(view_projection: Matrix) -> Self {
        let mut frustum = BoundingFrustum {
            view_projection: Matrix::IDENTITY,
            planes: [Plane::default(); 6],
            corners: [Vector3::ZERO; 8],
            corners_valid: false,
        };
        frustum.set_matrix(view_projection);
        frustum
    }

    pub fn corners(&mut self) -> &[Vector3; 8] {
        if !self.corners_valid {
            self.corners_valid = true;

            let mut ray = Self::compute_intersection_line(&self.planes[0], &self.planes[2]);
            self.corners[0] = Self::compute_intersection(&self.planes[4], &ray);
            self.corners[3] = Self::compute_intersection(&self.planes[5], &ray);

            ray = Self::compute_intersection_line(&self.planes[3], &self.planes[0]);
            self.corners[1] = Self::compute_intersection(&self.planes[4], &ray);
            self.corners[2] = Self::compute_intersection(&self.planes[5], &ray);

            ray = Self::compute_intersection_line(&self.planes[2], &self.planes[1]);
            self.corners[4] = Self::compute_intersection(&self.planes[4], &ray);
            self.corners[7] = Self::compute_intersection(&self.planes[5], &ray);

            ray = Self::compute_intersection_line(&self.planes[1], &self.planes[3]);
            self.corners[5] = Self::compute_intersection(&self.planes[4], &ray);
            self.corners[6] = Self::compute_intersection(&self.planes[5], &ray);
        }
        &self.corners
    }

    pub fn intersection_point(&self, point: Vector3) -> bool {
        for plane in &self.planes {
            if plane.normal.x * point.x + plane.normal.y * point.y + plane.normal.z * point.z + plane.d > 0.0 {
                return false;
            }
        }
        true
    }

    pub fn intersection_sphere(&self, sphere: &BoundingSphere) -> bool {
        for plane in &self.planes {
            if plane.normal.x * sphere.center.x + plane.normal.y * sphere.center.y
                + plane.normal.z * sphere.center.z + plane.d > sphere.radius
            {
                return false;
            }
        }
        true
    }

    pub fn intersection_box(&self, bb: &BoundingBox) -> bool {
        for plane in &self.planes {
            let x = plane.normal.x;
            let y = plane.normal.y;
            let z = plane.normal.z;
            let num = if x > 0.0 { bb.min.x } else { bb.max.x };
            let num2 = if y > 0.0 { bb.min.y } else { bb.max.y };
            let num3 = if z > 0.0 { bb.min.z } else { bb.max.z };
            if x * num + y * num2 + z * num3 + plane.d > 0.0 {
                return false;
            }
        }
        true
    }

    pub fn compute_intersection(plane: &Plane, ray: &Ray3) -> Vector3 {
        let s = (-plane.d - Vector3::dot(plane.normal, ray.position))
            / Vector3::dot(plane.normal, ray.direction);
        ray.position + ray.direction * s
    }

    pub fn compute_intersection_line(p1: &Plane, p2: &Plane) -> Ray3 {
        let direction = Vector3::cross(p1.normal, p2.normal);
        let d = direction.length_squared();
        let position = Vector3::cross(-p1.d * p2.normal + p2.d * p1.normal, direction) / d;
        Ray3::new(position, direction)
    }
}
