use std::fmt;
use std::u64;

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct TinySet(u64);

impl fmt::Debug for TinySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.into_iter().collect::<Vec<u32>>().fmt(f)
    }
}

pub struct TinySetIterator(TinySet);
impl Iterator for TinySetIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_lowest()
    }
}

impl IntoIterator for TinySet {
    type Item = u32;
    type IntoIter = TinySetIterator;
    fn into_iter(self) -> Self::IntoIter {
        TinySetIterator(self)
    }
}

impl TinySet {
    /// Returns an empty `TinySet`.
    pub fn empty() -> TinySet {
        TinySet(0u64)
    }

    /// Creates a new `TinySet` containing only one element
    /// within `[0; 64[`
    #[inline(always)]
    pub fn singleton(el: u32) -> TinySet {
        TinySet(1u64 << u64::from(el))
    }

    /// Insert a new element within [0..64[
    #[inline(always)]
    pub fn insert(self, el: u32) -> TinySet {
        self.union(TinySet::singleton(el))
    }

    /// Insert a new element within [0..64[
    #[inline(always)]
    pub fn insert_mut(&mut self, el: u32) -> bool {
        let old = *self;
        *self = old.insert(el);
        old != *self
    }

    /// Returns the union of two tinysets
    #[inline(always)]
    pub fn union(self, other: TinySet) -> TinySet {
        TinySet(self.0 | other.0)
    }

    /// Returns true iff the `TinySet` is empty.
    #[inline(always)]
    pub fn is_empty(self) -> bool {
        self.0 == 0u64
    }

    /// Returns the lowest element in the `TinySet`
    /// and removes it.
    #[inline(always)]
    pub fn pop_lowest(&mut self) -> Option<u32> {
        if self.is_empty() {
            None
        } else {
            let lowest = self.0.trailing_zeros() as u32;
            self.0 ^= TinySet::singleton(lowest).0;
            Some(lowest)
        }
    }

}

#[derive(Clone)]
pub struct BitSet {
    tinysets: Box<[TinySet]>,
    len: usize,
    max_value: u32,
}

fn num_buckets(max_val: u32) -> u32 {
    (max_val + 63u32) / 64u32
}

impl BitSet {
    /// Create a new `BitSet` that may contain elements
    /// within `[0, max_val[`.
    pub fn with_max_value(max_value: u32) -> BitSet {
        let num_buckets = num_buckets(max_value);
        let tinybisets = vec![TinySet::empty(); num_buckets as usize].into_boxed_slice();
        BitSet {
            tinysets: tinybisets,
            len: 0,
            max_value,
        }
    }

    /// Returns the number of elements in the `BitSet`.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Inserts an element in the `BitSet`
    pub fn insert(&mut self, el: u32) {
        // we do not check saturated els.
        let higher = el / 64u32;
        let lower = el % 64u32;
        self.len += if self.tinysets[higher as usize].insert_mut(lower) {
            1
        } else {
            0
        };
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item=u32> + 'a {
        self.tinysets
            .iter()
            .cloned()
            .enumerate()
            .flat_map(|(ord, tinyset)| {
                let offset = (ord *64) as  u32;
                tinyset.into_iter().map(move |el| offset + el)
            })
    }
}
