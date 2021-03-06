use crate::bitset::BitSet;
use crate::memory_arena::store;
use murmurhash32::murmurhash2;

use crate::memory_arena::{Addr, MemoryArena};
use byteorder::{ByteOrder, NativeEndian};
use std::iter;
use std::mem;
use std::slice;

// type UnorderedTermId = u32;

/// Returns the actual memory size in bytes
/// required to create a table of size $2^num_bits$.
pub fn compute_table_size(num_bits: usize) -> usize {
    (1 << num_bits) * mem::size_of::<KeyValue>()
}

/// `KeyValue` is the item stored in the hash table.
/// The key is actually a `BytesRef` object stored in an external heap.
/// The `value_addr` also points to an address in the heap.
///
/// The key and the value are actually stored contiguously.
/// For this reason, the (start, stop) information is actually redundant
/// and can be simplified in the future
#[derive(Copy, Clone)]
struct KeyValue {
    key_value_addr: Addr,
    hash: u32,
    // unordered_term_id: UnorderedTermId,
}

impl Default for KeyValue {
    fn default() -> Self {
        KeyValue {
            key_value_addr: Addr::null_pointer(),
            hash: 0u32,
            // unordered_term_id: UnorderedTermId::default(),
        }
    }
}

impl KeyValue {
    fn is_empty(self) -> bool {
        self.key_value_addr.is_null()
    }
}

/// Customized `HashMap` with string keys
///
/// This `HashMap` takes String as keys. Keys are
/// stored in a user defined heap.
///
/// The quirky API has the benefit of avoiding
/// the computation of the hash of the key twice,
/// or copying the key as long as there is no insert.
///
pub struct TermHashMap {
    table: Box<[KeyValue]>,
    pub heap: MemoryArena,
    mask: usize,
    occupied: BitSet,
    len: usize,
}

struct QuadraticProbing {
    hash: usize,
    i: usize,
    mask: usize,
}

impl QuadraticProbing {
    fn compute(hash: usize, mask: usize) -> QuadraticProbing {
        QuadraticProbing { hash, i: 0, mask }
    }

    #[inline]
    fn next_probe(&mut self) -> usize {
        self.i += 1;
        (self.hash + self.i) & self.mask
    }
}

pub struct Iter<'a> {
    hashmap: &'a TermHashMap,
    inner: slice::Iter<'a, usize>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a [u8], Addr);//, UnorderedTermId);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().cloned().map(move |bucket: usize| {
            let kv = self.hashmap.table[bucket];
            let (key, offset): (&'a [u8], Addr) = self.hashmap.get_key_value(kv.key_value_addr);
            (key, offset)//, kv.unordered_term_id)
        })
    }
}

impl TermHashMap {
    pub fn new(num_bucket_power_of_2: usize) -> TermHashMap {
        let heap = MemoryArena::new();
        let table_size = 1 << num_bucket_power_of_2;
        let occupied: BitSet = BitSet::with_max_value(table_size as u32);
        let table: Vec<KeyValue> = iter::repeat(KeyValue::default()).take(table_size).collect();
        TermHashMap {
            table: table.into_boxed_slice(),
            heap,
            mask: table_size - 1,
            occupied,
            len: 0,
        }
    }

    fn probe(&self, hash: u32) -> QuadraticProbing {
        QuadraticProbing::compute(hash as usize, self.mask)
    }

    pub fn mem_usage(&self) -> usize {
        self.table.len() * mem::size_of::<KeyValue>()
    }

    fn is_saturated(&self) -> bool {
        self.table.len() < self.occupied.len() * 2
    }

    #[inline(always)]
    fn get_key_value(&self, addr: Addr) -> (&[u8], Addr) {
        let data = self.heap.slice_from(addr);
        let key_bytes_len = NativeEndian::read_u16(data) as usize;
        let key_bytes: &[u8] = &data[2..][..key_bytes_len];
        (key_bytes, addr.offset(2u32 + key_bytes_len as u32))
    }

    #[inline(always)]
    fn get_value_addr_if_key_match(&self, target_key: &[u8], addr: Addr) -> Option<Addr> {
        let (stored_key, value_addr) = self.get_key_value(addr);
        if stored_key == target_key {
            Some(value_addr)
        } else {
            None
        }
    }

    fn set_bucket(&mut self, hash: u32, key_value_addr: Addr, bucket: usize) {//-> UnorderedTermId {
        self.occupied.insert(bucket as u32);
        // let unordered_term_id = self.len as UnorderedTermId;
        self.len += 1;
        self.table[bucket] = KeyValue {
            key_value_addr,
            hash,
            // unordered_term_id,
        };
        // unordered_term_id
    }

    fn resize(&mut self) {
        let new_len = self.table.len() * 2;
        let mask = new_len - 1;
        self.mask = mask;
        let new_table = vec![KeyValue::default(); new_len].into_boxed_slice();
        let old_table = mem::replace(&mut self.table, new_table);
        let old_occupied = mem::replace(&mut self.occupied, BitSet::with_max_value(new_len as u32));
        for old_pos in old_occupied.iter() {
            let key_value: KeyValue = old_table[old_pos as usize];
            let mut probe = QuadraticProbing::compute(key_value.hash as usize, mask);
            loop {
                let bucket = probe.next_probe();
                if self.table[bucket].is_empty() {
                    self.occupied.insert(bucket as u32);
                    self.table[bucket] = key_value;
                    break;
                }
            }
        }
    }

    /// `update` create a new entry for a given key if it does not exists
    /// or updates the existing entry.
    ///
    /// The actual logic for this update is define in the the `updater`
    /// argument.
    ///
    /// If the key is not present, `updater` will receive `None` and
    /// will be in charge of returning a default value.
    /// If the key already as an associated value, then it will be passed
    /// `Some(previous_value)`.
    pub fn mutate_or_create<S, V, TMutator>(
        &mut self,
        key: S,
        mut updater: TMutator,
    ) //-> UnorderedTermId
    where
        S: AsRef<[u8]>,
        V: Copy + 'static,
        TMutator: FnMut(Option<V>) -> V,
    {
        if self.is_saturated() {
            self.resize();
        }
        let key_bytes: &[u8] = key.as_ref();
        let hash = murmurhash2(key.as_ref());
        let mut probe = self.probe(hash);
        loop {
            let bucket = probe.next_probe();
            let kv: KeyValue = self.table[bucket];
            if kv.is_empty() {
                // The key does not exists yet.
                let val = updater(None);
                let num_bytes =
                    std::mem::size_of::<u16>() + key_bytes.len() + std::mem::size_of::<V>();
                let key_addr = self.heap.allocate_space(num_bytes);
                {
                    let data = self.heap.slice_mut(key_addr, num_bytes);
                    NativeEndian::write_u16(data, key_bytes.len() as u16);
                    let stop = 2 + key_bytes.len();
                    data[2..stop].copy_from_slice(key_bytes);
                    store(&mut data[stop..], val);
                }
                return self.set_bucket(hash, key_addr, bucket);
            } else if kv.hash == hash {
                if let Some(val_addr) =
                    self.get_value_addr_if_key_match(key_bytes, kv.key_value_addr)
                {
                    let v = self.heap.read(val_addr);
                    let new_v = updater(Some(v));
                    self.heap.write_at(val_addr, new_v);
                    return; //kv.unordered_term_id;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::TermHashMap;
    

    #[test]
    fn test_hash_map() {
        let mut hash_map: TermHashMap = TermHashMap::new(18);
        {
            hash_map.mutate_or_create("abc", |opt_val: Option<u32>| {
                assert_eq!(opt_val, None);
                3u32
            });
        }
        {
            hash_map.mutate_or_create("abcd", |opt_val: Option<u32>| {
                assert_eq!(opt_val, None);
                4u32
            });
        }
        {
            hash_map.mutate_or_create("abc", |opt_val: Option<u32>| {
                assert_eq!(opt_val, Some(3u32));
                5u32
            });
        }

    }
}
