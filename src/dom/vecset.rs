use crate::VecMap;
use ownable::traits::{IntoOwned, ToBorrowed, ToOwned};
#[cfg(feature = "test")]
use serde::{Serialize, Serializer};
use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};
use std::iter::{FromIterator, Map};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct VecSet<K>(pub VecMap<K, ()>);

impl<K> VecSet<K> {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> Iter<'_, K> {
        Iter(self.0.iter().map(|(k, _)| k))
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Eq + Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.0.contains_key(key)
    }

    pub fn insert(&mut self, key: K) -> bool
    where
        K: Eq,
    {
        self.0.insert(key, ()).is_none()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> bool
    where
        K: Eq + Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.0.remove(key).is_some()
    }
}

impl<K> Debug for VecSet<K>
where
    K: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.0.iter()).finish()
    }
}

impl<K> FromIterator<K> for VecSet<K> {
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        Self(VecMap::from_iter(iter.into_iter().map(|k| (k, ()))))
    }
}

#[allow(clippy::type_complexity)]
pub struct Iter<'a, K>(pub Map<crate::dom::vecmap::Iter<'a, K, ()>, fn((&'a K, &'a ())) -> &'a K>);

impl<'a, K> Iterator for Iter<'a, K> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[allow(clippy::type_complexity)]
pub struct IntoIter<K>(pub Map<crate::dom::vecmap::IntoIter<K, ()>, fn((K, ())) -> K>);

impl<K> IntoIterator for VecSet<K> {
    type Item = K;
    type IntoIter = IntoIter<K>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter().map(|(k, _)| k))
    }
}

impl<K> Iterator for IntoIter<K> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a, K: ToBorrowed<'a>> ToBorrowed<'a> for VecSet<K> {
    fn to_borrowed(&'a self) -> Self {
        VecSet(VecMap(self.iter().map(|k| (k.to_borrowed(), ())).collect()))
    }
}

impl<K: ToOwned> ToOwned for VecSet<K> {
    type Owned = VecSet<K::Owned>;
    fn to_owned(&self) -> Self::Owned {
        VecSet(VecMap(self.iter().map(|k| (k.to_owned(), ())).collect()))
    }
}

impl<K: IntoOwned> IntoOwned for VecSet<K> {
    type Owned = VecSet<K::Owned>;
    fn into_owned(self) -> Self::Owned {
        VecSet(VecMap(
            self.into_iter().map(|k| (k.into_owned(), ())).collect(),
        ))
    }
}

#[cfg(feature = "test")]
impl<K> Serialize for VecSet<K>
where
    K: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.0.iter().map(|(k, _)| k))
    }
}
