use ownable::traits::{IntoOwned, ToBorrowed, ToOwned};
#[cfg(feature = "test")]
use serde::{Serialize, Serializer};
use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};
use std::iter::{FromIterator, Map};
use std::mem;
use std::ops::Index;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct VecMap<K, V>(pub Vec<(K, V)>);

impl<K, V> VecMap<K, V> {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter(self.0.iter().map(|kv| (&kv.0, &kv.1)))
    }

    fn position<Q>(&self, key: &Q) -> Option<usize>
    where
        K: Eq + Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.0.iter().position(|(k, _)| k.borrow() == key)
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Eq + Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.position(key).is_some()
    }

    pub fn insert(&mut self, key: K, mut value: V) -> Option<V>
    where
        K: Eq,
    {
        match self.position(&key) {
            None => {
                self.0.push((key, value));
                None
            }
            Some(i) => {
                mem::swap(&mut value, &mut unsafe { self.0.get_unchecked_mut(i) }.1);
                Some(value)
            }
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Eq + Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.position(key)
            .map(|i| &unsafe { self.0.get_unchecked(i) }.1)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Eq + Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.position(key)
            .map(move |i| &mut unsafe { self.0.get_unchecked_mut(i) }.1)
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Eq + Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.position(key).map(|i| self.0.remove(i).1)
    }
}

impl<K, V> Debug for VecMap<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

#[allow(clippy::type_complexity)]
pub struct Iter<'a, K, V>(pub Map<std::slice::Iter<'a, (K, V)>, fn(&'a (K, V)) -> (&K, &V)>);

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<K, V> FromIterator<(K, V)> for VecMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

pub struct IntoIter<K, V>(pub std::vec::IntoIter<(K, V)>);

impl<K, V> IntoIterator for VecMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a, K: ToBorrowed<'a>, V: ToBorrowed<'a>> ToBorrowed<'a> for VecMap<K, V> {
    fn to_borrowed(&'a self) -> Self {
        VecMap(
            self.iter()
                .map(|(k, v)| (k.to_borrowed(), v.to_borrowed()))
                .collect(),
        )
    }
}

impl<K: ToOwned, V: ToOwned> ToOwned for VecMap<K, V> {
    type Owned = VecMap<K::Owned, V::Owned>;
    fn to_owned(&self) -> Self::Owned {
        VecMap(
            self.iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect(),
        )
    }
}

impl<K: IntoOwned, V: IntoOwned> IntoOwned for VecMap<K, V> {
    type Owned = VecMap<K::Owned, V::Owned>;
    fn into_owned(self) -> Self::Owned {
        VecMap(
            self.into_iter()
                .map(|(k, v)| (k.into_owned(), v.into_owned()))
                .collect(),
        )
    }
}

impl<K, Q, V> Index<&Q> for VecMap<K, V>
where
    K: Eq + Borrow<Q>,
    Q: Eq + ?Sized,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).expect("no entry found for key")
    }
}

#[cfg(feature = "test")]
impl<K, V> Serialize for VecMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(self.iter())
    }
}
