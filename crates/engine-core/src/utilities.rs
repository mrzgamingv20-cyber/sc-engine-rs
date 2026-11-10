pub fn swap<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b);
}

pub fn size_of<T>() -> usize {
    std::mem::size_of::<T>()
}
