use super::point2::Point2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rectangle {
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    pub const EMPTY: Rectangle = Rectangle { left: 0, top: 0, width: 0, height: 0 };

    #[inline]
    pub fn new(left: i32, top: i32, width: i32, height: i32) -> Self {
        Rectangle { left, top, width, height }
    }

    #[inline]
    pub fn location(&self) -> Point2 {
        Point2::new(self.left, self.top)
    }

    #[inline]
    pub fn size(&self) -> Point2 {
        Point2::new(self.width, self.height)
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
    pub fn contains(&self, p: Point2) -> bool {
        p.x >= self.left && p.x < self.left + self.width
            && p.y >= self.top && p.y < self.top + self.height
    }

    #[inline]
    pub fn intersects(&self, r: &Rectangle) -> bool {
        let num = self.left.max(r.left);
        let num2 = self.top.max(r.top);
        let num3 = self.right().min(r.right());
        let num4 = self.bottom().min(r.bottom());
        num3 > num && num4 > num2
    }

    pub fn intersection(r1: &Rectangle, r2: &Rectangle) -> Rectangle {
        let left = r1.left.max(r2.left);
        let top = r1.top.max(r2.top);
        let right = r1.right().min(r2.right());
        let bottom = r1.bottom().min(r2.bottom());
        if right <= left || bottom <= top {
            Rectangle::EMPTY
        } else {
            Rectangle::new(left, top, right - left, bottom - top)
        }
    }

    pub fn union(r1: &Rectangle, r2: &Rectangle) -> Rectangle {
        let left = r1.left.min(r2.left);
        let top = r1.top.min(r2.top);
        let right = r1.right().max(r2.right());
        let bottom = r1.bottom().max(r2.bottom());
        Rectangle::new(left, top, right - left, bottom - top)
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{},{}", self.left, self.top, self.width, self.height)
    }
}
