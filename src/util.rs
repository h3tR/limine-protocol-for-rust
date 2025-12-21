pub struct PointerSlice<T: 'static> {
    inner: &'static [*const T],
}

impl<T> From<&'static [*const T]> for PointerSlice<T> {
    fn from(slice: &'static [*const T]) -> Self {
        Self { inner: slice }
    }
}

impl<T: 'static> PointerSlice<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter().map(|&ptr| unsafe { &*ptr })
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index).map(|&ptr| unsafe { &*ptr })
    }
}