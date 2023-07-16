rust
#![feature(entry_insert, generic_associated_types)]

use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

pub trait Push {
    type Item;
    type Output<'a>
    where
        Self: 'a;
    fn push<'a>(&'a mut self, item: Self::Item) -> Self::Output<'a>;
}

impl<K, V, S> Push for HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Item = (K, V);
    type Output<'a>
    where
        Self: 'a,
    = &'a mut V;
    fn push<'a>(&'a mut self, (key, value): (K, V)) -> &'a mut V {
        self.entry(key).insert(value).into_mut()
    }
}
