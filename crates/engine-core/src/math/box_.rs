use super::point3::Point3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Box {
    pub left: i32,
    pub top: i32,
    pub near: i32,
    pub width: i32,
    pub height: i32,
    pub depth: i32,
}

impl Box {
    pub const EMPTY: Box = Box { left: 0, top: 0, near: 0, width: 0, height: 0, depth: 0 };

    #[inline]
    pub fn new(left: i32, top: i32, near: i32, width: i32, height: i32, depth: i32) -> Self {
        Box { left, top, near, width, height, depth }
    }

    #[inline]
    pub fn location(&self) -> Point3 {
        Point3::new(self.left, self.top, self.near)
    }

    #[inline]
    pub fn size(&self) -> Point3 {
        Point3::new(self.width, self.height, self.depth)
    }

    #[inline]
    pub fn right(&self) -> i32 {
        self.left + self.width
    }

    #[inline]
    pub fn bottom(&self) -> i32 {
        self.top + self.height
    }

    #[inline]
    pub fn far(&self) -> i32 {
        self.near + self.depth
    }

    #[inline]
    pub fn contains(&self, p: Point3) -> bool {
        p.x >= self.left && p.x < self.left + self.width
            && p.y >= self.top && p.y < self.top + self.height
            && p.z >= self.near && p.z < self.near + self.depth
    }

    pub fn intersection(b1: &Box, b2: &Box) -> Box {
        let left = b1.left.max(b2.left);
        let top = b1.top.max(b2.top);
        let near = b1.near.min(b2.near);
        let right = b1.right().min(b2.right());
        let bottom = b1.bottom().min(b2.bottom());
        let far = b1.far().min(b2.far());
        if right <= left || bottom <= top || far <= near {
            Box::EMPTY
        } else {
            Box::new(left, top, near, right - left, bottom - top, far - near)
        }
    }

    pub fn union(b1: &Box, b2: &Box) -> Box {
        let left = b1.left.min(b2.left);
        let top = b1.top.min(b2.top);
        let near = b1.near.min(b2.near);
        let right = b1.right().max(b2.right());
        let bottom = b1.bottom().max(b2.bottom());
        let far = b1.far().max(b2.far());
        Box::new(left, top, near, right - left, bottom - top, far - near)
    }
}

impl std::fmt::Display for Box {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{},{},{},{}",
            self.left, self.top, self.near, self.width, self.height, self.depth)
    }
}
