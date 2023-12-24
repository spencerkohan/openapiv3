use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use crate::ReferenceOr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefOrItemMap<T>(IndexMap<String, ReferenceOr<T>>);

impl<T> RefOrItemMap<T> {
    pub fn new(map: IndexMap<String, ReferenceOr<T>>) -> Self {
        RefOrItemMap(map)
    }
}

impl<T> std::ops::Deref for RefOrItemMap<T> {
    type Target = IndexMap<String, ReferenceOr<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for RefOrItemMap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> RefOrItemMap<T> {
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<ReferenceOr<T>>) {
        let key = key.into();
        let value = value.into();
        self.0.insert(key, value);
    }
}

impl<T> std::iter::FromIterator<(String, ReferenceOr<T>)> for RefOrItemMap<T> {
    fn from_iter<I: IntoIterator<Item = (String, ReferenceOr<T>)>>(iter: I) -> Self {
        RefOrItemMap(IndexMap::from_iter(iter))
    }
}

impl<T> Default for RefOrItemMap<T> {
    fn default() -> Self {
        RefOrItemMap(IndexMap::default())
    }
}

impl<T> Into<IndexMap<String, ReferenceOr<T>>> for RefOrItemMap<T> {
    fn into(self) -> IndexMap<String, ReferenceOr<T>> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oa_ref_map_insert_coercion() {
        let mut s: RefOrItemMap<usize> = RefOrItemMap(IndexMap::new());
        s.insert("a", 1);
    }
}