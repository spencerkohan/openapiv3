use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::RefOr;

pub type RefOrMap<T> = RefMap<T>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefMap<T>(IndexMap<String, RefOr<T>>);

impl<T> RefOrMap<T> {
    pub fn new() -> Self {
        RefMap(IndexMap::new())
    }

    /// Directly get the inner struct of a RefOr::Item
    pub fn get2(&self, key: &str) -> Option<&T> {
        let v = self.0.get(key);
        v.and_then(|v| v.as_item())
    }

    /// Directly get_mut the inner struct of a RefOr::Item
    pub fn get_mut2(&mut self, key: &str) -> Option<&mut T> {
        let v = self.0.get_mut(key);
        v.and_then(|v| v.as_mut())
    }

    /// Directly lookup the inner struct of a RefOr::Item, panicking on not found
    pub fn index2(&self, key: &str) -> &T {
        self.get2(key).expect("key not found")
    }

    /// Directly lookup (mut) the inner struct of a RefOr::Item, panicking on not found
    pub fn index_mut2(&mut self, key: &str) -> &mut T {
        self.get_mut2(key).expect("key not found")
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
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<RefOr<T>>) -> Option<RefOr<T>> {
        let key = key.into();
        let value = value.into();
        self.0.insert(key, value)
    }
}

impl<T> std::iter::FromIterator<(String, RefOr<T>)> for RefOrMap<T> {
    fn from_iter<I: IntoIterator<Item = (String, RefOr<T>)>>(iter: I) -> Self {
        RefMap(IndexMap::from_iter(iter))
    }
}

impl<T> IntoIterator for RefMap<T> {
    type Item = (String, RefOr<T>);
    type IntoIter = indexmap::map::IntoIter<String, RefOr<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a RefMap<T> {
    type Item = (&'a String, &'a RefOr<T>);
    type IntoIter = indexmap::map::Iter<'a, String, RefOr<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut RefMap<T> {
    type Item = (&'a String, &'a mut RefOr<T>);
    type IntoIter = indexmap::map::IterMut<'a, String, RefOr<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<T> Default for RefMap<T> {
    fn default() -> Self {
        RefMap(IndexMap::default())
    }
}

impl<T> Into<IndexMap<String, RefOr<T>>> for RefOrMap<T> {
    fn into(self) -> IndexMap<String, RefOr<T>> {
        self.0
    }
}

impl<T> From<IndexMap<String, RefOr<T>>> for RefMap<T> {
    fn from(map: IndexMap<String, RefOr<T>>) -> Self {
        RefMap(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oa_ref_map_insert_coercion() {
        let mut s: RefOrMap<usize> = RefMap(IndexMap::new());
        s.insert("a", 1);
    }
}
