pub struct DynamicArray<T> {
    array: Vec<T>,
}

impl<T: Clone + Default> DynamicArray<T> {
    const MIN_CAPACITY: usize = 4;

    pub fn new() -> Self {
        DynamicArray { array: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        DynamicArray { array: Vec::with_capacity(capacity) }
    }

    pub fn from_slice(items: &[T]) -> Self {
        let mut arr = DynamicArray::new();
        arr.add_range(items);
        arr
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.array.capacity()
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.array.len()
    }

    #[inline]
    pub fn array(&self) -> &[T] {
        &self.array
    }

    #[inline]
    pub fn array_mut(&mut self) -> &mut [T] {
        &mut self.array
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.array.get(index)
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.array.get_mut(index)
    }

    pub fn index_of(&self, item: &T) -> isize
    where
        T: PartialEq,
    {
        self.array.iter().position(|x| x == item).map(|i| i as isize).unwrap_or(-1)
    }

    pub fn add(&mut self, item: T) {
        self.array.push(item);
    }

    pub fn add_range(&mut self, items: &[T])
    where
        T: Clone,
    {
        self.array.extend_from_slice(items);
    }

    pub fn remove(&mut self, item: &T) -> bool
    where
        T: PartialEq,
    {
        if let Some(pos) = self.array.iter().position(|x| x == item) {
            self.array.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn remove_at(&mut self, index: usize) -> T {
        self.array.remove(index)
    }

    pub fn remove_at_end(&mut self) -> T {
        self.array.pop().expect("DynamicArray is empty")
    }

    pub fn remove_all(&mut self, mut pred: impl FnMut(&T) -> bool) -> usize {
        let old_len = self.array.len();
        self.array.retain(|x| !pred(x));
        old_len - self.array.len()
    }

    pub fn remove_range(&mut self, index: usize, count: usize) {
        assert!(index + count <= self.array.len());
        self.array.drain(index..index + count);
    }

    pub fn insert(&mut self, index: usize, item: T) {
        self.array.insert(index, item);
    }

    pub fn clear(&mut self) {
        self.array.clear();
    }

    pub fn reverse(&mut self) {
        self.array.reverse();
    }

    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.array.sort();
    }

    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        self.array.sort_by(compare);
    }

    pub fn contains(&self, item: &T) -> bool
    where
        T: PartialEq,
    {
        self.array.contains(item)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.array.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.array.iter_mut()
    }
}

impl<T: Clone + Default> Default for DynamicArray<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Default> From<&[T]> for DynamicArray<T> {
    fn from(items: &[T]) -> Self {
        Self::from_slice(items)
    }
}
