//! no_std HashMap/HashSet wrappers around IndexMap/IndexSet.

use crate::{IndexMap, IndexSet};
use core::hash::Hash;
use core::iter::FromIterator;
use core::ops::{Deref, DerefMut};

#[derive(Clone, Debug)]
pub struct HashMap<K, V>(IndexMap<K, V>);

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self(IndexMap::default())
    }
}

impl<K, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Deref for HashMap<K, V> {
    type Target = IndexMap<K, V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> DerefMut for HashMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V> IntoIterator for HashMap<K, V> {
    type Item = (K, V);
    type IntoIter = indexmap::map::IntoIter<K, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<K: Hash + Eq, V> FromIterator<(K, V)> for HashMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}

impl<K: Hash + Eq, V> Extend<(K, V)> for HashMap<K, V> {
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        self.0.extend(iter)
    }
}

#[derive(Clone, Debug)]
pub struct HashSet<T>(IndexSet<T>);

impl<T> HashSet<T> {
    pub fn new() -> Self {
        Self(IndexSet::default())
    }
}

impl<T> Default for HashSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Deref for HashSet<T> {
    type Target = IndexSet<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for HashSet<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> IntoIterator for HashSet<T> {
    type Item = T;
    type IntoIter = indexmap::set::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Hash + Eq> FromIterator<T> for HashSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(IndexSet::from_iter(iter))
    }
}

impl<T: Hash + Eq> Extend<T> for HashSet<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter)
    }
}
