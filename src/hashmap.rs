//! Implement Fallible HashMap
use crate::TryReserveError;
use core::hash::Hash;

#[derive(Default)]
pub struct TryHashMap<K, V> {
    #[cfg(feature = "unstable")]
    inner: std::collections::HashMap<K, V>,
    #[cfg(not(feature = "unstable"))]
    inner: hashbrown::hash_map::HashMap<K, V>,
}

impl<K, V> TryHashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: core::borrow::Borrow<Q>,
        Q: Hash + Eq,
    {
        self.inner.get(k)
    }

    pub fn insert(&mut self, k: K, v: V) -> Result<Option<V>, TryReserveError> {
        self.reserve(if self.inner.capacity() == 0 { 4 } else { 1 })?;
        Ok(self.inner.insert(k, v))
    }

    fn reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve(additional)
    }
}

#[test]
fn tryhashmap_oom() {
    match TryHashMap::<char, char>::default().reserve(core::usize::MAX) {
        Ok(_) => panic!("it should be OOM"),
        _ => (),
    }
}
