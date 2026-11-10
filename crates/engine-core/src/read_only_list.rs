pub struct ReadOnlyList<'a, T> {
    list: &'a [T],
}

impl<'a, T> ReadOnlyList<'a, T> {
    #[inline]
    pub fn new(list: &'a [T]) -> Self {
        ReadOnlyList { list }
    }

    #[inline]
    pub fn empty() -> Self {
        ReadOnlyList { list: &[] }
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.list.len()
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.list.get(index)
    }

    #[inline]
    pub fn contains(&self, item: &T) -> bool
    where
        T: PartialEq,
    {
        self.list.contains(item)
    }

    #[inline]
    pub fn index_of(&self, item: &T) -> isize
    where
        T: PartialEq,
    {
        self.list.iter().position(|x| x == item).map(|i| i as isize).unwrap_or(-1)
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'a, T> {
        self.list.iter()
    }
}

impl<'a, T> From<&'a [T]> for ReadOnlyList<'a, T> {
    fn from(list: &'a [T]) -> Self {
        Self::new(list)
    }
}
