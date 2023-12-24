use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use crate::RefOr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefOrMap<T>(IndexMap<String, RefOr<T>>);

impl<T> RefOrMap<T> {
    pub fn new(map: IndexMap<String, RefOr<T>>) -> Self {
        RefOrMap(map)
    }
}

impl<T> std::ops::Deref for RefOrMap<T> {
    type Target = IndexMap<String, RefOr<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for RefOrMap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> RefOrMap<T> {
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<RefOr<T>>) {
        let key = key.into();
        let value = value.into();
        self.0.insert(key, value);
    }
}

impl<T> std::iter::FromIterator<(String, RefOr<T>)> for RefOrMap<T> {
    fn from_iter<I: IntoIterator<Item = (String, RefOr<T>)>>(iter: I) -> Self {
        RefOrMap(IndexMap::from_iter(iter))
    }
}

impl<T> Default for RefOrMap<T> {
    fn default() -> Self {
        RefOrMap(IndexMap::default())
    }
}

impl<T> Into<IndexMap<String, RefOr<T>>> for RefOrMap<T> {
    fn into(self) -> IndexMap<String, RefOr<T>> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oa_ref_map_insert_coercion() {
        let mut s: RefOrMap<usize> = RefOrMap(IndexMap::new());
        s.insert("a", 1);
    }
}