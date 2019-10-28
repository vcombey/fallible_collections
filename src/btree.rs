pub mod map;
pub use map::BTreeMap;

pub mod set;
pub use set::BTreeSet;

mod node;
mod search;
use alloc::collections::CollectionAllocErr;

#[doc(hidden)]
trait Recover<Q: ?Sized> {
    type Key;

    fn get(&self, key: &Q) -> Option<&Self::Key>;
    fn take(&mut self, key: &Q) -> Option<Self::Key>;
    fn replace(&mut self, key: Self::Key) -> Result<Option<Self::Key>, CollectionAllocErr>;
}
