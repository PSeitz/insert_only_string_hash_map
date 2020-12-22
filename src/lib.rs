/*! Stores values for strings in a Hashmap in a fast and compact way.

Good to count strings and assign ids to them or similar. Address space of string data is limited to u32::MAX (4GB).
string data is size in bytes of all uniquely inserted strings + string length metadata per string.

# Examples
```
use inohashmap::StringHashMap;
let mut hashmap = StringHashMap::<u32>::new();
let val = hashmap.get_or_create("blub1", 0);
assert_eq!(*val, 0);
*val += 1;

let val = hashmap.get_or_create("blub2", 2);
assert_eq!(*val, 2);

```

*/

use crate::bytesref::BytesRef;
use crate::hasher::fnv32a_yoshimitsu_hasher;
use core::fmt::Debug;
use vint32::{decode_varint_slice, encode_varint_into};
mod bytesref;
pub mod hasher;

#[derive(Debug)]
pub struct StringHashMap<T> {
    /// contains string in compressed format
    pub(crate) string_data: Vec<u8>,
    /// pointer to string data and value
    pub(crate) table: Vec<TableEntry<T>>,
    bitshift: usize,
    pub occupied: usize,
    mask: u32,
}

impl<T: Default + Clone + Debug> Default for StringHashMap<T> {
    fn default() -> Self {
        StringHashMap::with_power_of_two_size(10)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct TableEntry<T> {
    value: T,
    pointer: BytesRef,
}

impl<T: Default + Clone + Debug> StringHashMap<T> {
    #[inline]
    pub fn with_power_of_two_size(power_of_two: usize) -> Self {
        let shift = power_of_two - 1;
        let mut table = vec![];
        table.resize(1 << shift, TableEntry::default());
        StringHashMap {
            string_data: Vec::with_capacity((1 << shift) * 2),
            mask: table.len() as u32 - 1,
            table,
            bitshift: 32 - power_of_two,
            occupied: 0,
        }
    }
    #[inline]
    pub fn new() -> Self {
        Self::with_power_of_two_size(10)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.occupied
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.string_data.shrink_to_fit();
        self.table.shrink_to_fit();
    }

    #[inline]
    pub fn get(&mut self, el: &str) -> Option<&T> {
        let mut probe = self.get_probe(el);
        let mut hash = probe.next_probe() as usize;

        loop {
            let entry = self.get_entry(hash);
            if entry.pointer.is_null() {
                return None;
            } else if self.read_string(entry.pointer) == el {
                return Some(&self.get_entry(hash as usize).value);
            }
            hash = probe.next_probe() as usize;
        }
    }
    #[inline]
    pub fn get_mut(&mut self, el: &str) -> Option<&mut T> {
        let mut probe = self.get_probe(el);
        let mut hash = probe.next_probe() as usize;

        loop {
            let entry = self.get_entry(hash);
            if entry.pointer.is_null() {
                return None;
            } else if self.read_string(entry.pointer) == el {
                return Some(&mut self.get_entry_mut(hash as usize).value);
            }
            hash = probe.next_probe() as usize;
        }
    }

    #[inline]
    pub fn get_or_create(&mut self, el: &str, value: T) -> &mut T {
        // check load factor, resize when 0.5
        // if self.occupied as f32 * 1.5 > self.table.len() as f32 {
        if self.occupied as f32 * 1.5 > self.table.len() as f32 {
            self.resize();
        }
        let mut probe = self.get_probe(el);
        let mut hash = probe.next_probe() as usize;

        loop {
            let entry = self.get_entry(hash);
            if entry.pointer.is_null() {
                self.occupied += 1;
                let inserted_value = self.put_in_bucket(hash as usize, el, value);
                return &mut inserted_value.value;
            } else if self.read_string(entry.pointer) == el {
                return &mut self.get_entry_mut(hash as usize).value;
            }
            hash = probe.next_probe() as usize;
        }
    }

    #[inline]
    fn get_probe(&self, el: &str) -> QuadraticProbing {
        let hash = fnv32a_yoshimitsu_hasher(el.as_bytes());
        let hash = hash >> self.bitshift;
        let probe = QuadraticProbing::compute(hash, self.mask);
        probe
    }

    #[inline]
    fn put_entry_resize(&mut self, el: &str, new_entry: TableEntry<T>) {
        let mut probe = self.get_probe(el);
        let mut hash = probe.next_probe();
        loop {
            let entry = self.get_entry_mut(hash as usize);
            if entry.pointer.is_null() {
                entry.pointer = new_entry.pointer;
                entry.value = new_entry.value;
                return;
            }
            hash = probe.next_probe();
        }
    }

    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.table
            .iter()
            .filter(|entry| !entry.pointer.is_null())
            .map(|entry| &entry.value)
    }
    #[inline]
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.table
            .iter_mut()
            .filter(|entry| !entry.pointer.is_null())
            .map(|entry| &mut entry.value)
    }
    #[inline]
    pub fn keys(&self) -> KeyIterator<'_, T> {
        KeyIterator { map: self, pos: 0 }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&str, &T)> {
        self.table
            .iter()
            .filter(|entry| !entry.pointer.is_null())
            .map(move |entry| (self.read_string(entry.pointer), &entry.value))
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&str, &mut T)> {
        // You bested me borrow checker
        // Cast should be fine, since self lives als long as the iter and all data accessed in read_string is immutable
        // I don't know why but mutable access doesn't work here without errors
        let cheated_self = unsafe { std::mem::transmute::<&mut Self, &Self>(self) };
        self.table
            .iter_mut()
            .filter(|entry| !entry.pointer.is_null())
            .map(move |entry| {
                let text = cheated_self.read_string(entry.pointer);
                (text, &mut entry.value)
            })
    }

    #[inline]
    fn get_entry(&self, hash: usize) -> &TableEntry<T> {
        unsafe { self.table.get_unchecked(hash) }
    }
    #[inline]
    fn get_entry_mut(&mut self, hash: usize) -> &mut TableEntry<T> {
        unsafe { self.table.get_unchecked_mut(hash as usize) }
    }

    /// Doubles the size of the table
    /// Creates a new table and moves all entries to the new table
    #[cold]
    fn resize(&mut self) {
        let mut table: Vec<TableEntry<T>> = vec![];
        table.resize(self.table.len() * 2, TableEntry::default());
        self.mask = table.len() as u32 - 1;

        std::mem::swap(&mut self.table, &mut table);
        self.bitshift -= 1;
        for entry in table.into_iter().filter(|x| !x.pointer.is_null()) {
            let text = self.read_string(entry.pointer);
            // casting away lifetime of text
            // Since string_data will not be altered in put_entry_resize
            let text = unsafe { std::mem::transmute::<&str, &'static str>(text) };
            self.put_entry_resize(text, entry);
        }
    }

    #[inline]
    fn put_in_bucket(&mut self, hash: usize, el: &str, value: T) -> &mut TableEntry<T> {
        let pos = BytesRef(self.string_data.len() as u32);

        encode_varint_into(&mut self.string_data, el.len() as u32);

        self.string_data.extend_from_slice(el.as_bytes());
        // unsafe {
        //     self.string_data.reserve(el.len());
        //     let target = self.string_data.as_mut_ptr().add(self.string_data.len());
        //     std::ptr::copy_nonoverlapping(el.as_bytes().as_ptr(), target, el.as_bytes().len());
        //     self.string_data.set_len(self.string_data.len()+ el.len() );
        // };

        let entry = self.get_entry_mut(hash);
        *entry = TableEntry {
            value,
            pointer: pos,
        };
        entry
    }

    #[inline]
    pub(crate) fn read_string(&self, pos: BytesRef) -> &str {
        let mut pos = pos.addr() as usize;
        let length_string = decode_varint_slice(&self.string_data, &mut pos).unwrap();
        unsafe {
            std::str::from_utf8_unchecked(
                &self
                    .string_data
                    .get_unchecked(pos..pos + length_string as usize),
            )
        }
    }
}

#[derive(Debug)]
pub struct KeyIterator<'a, T> {
    pub map: &'a StringHashMap<T>,
    pos: usize,
}

impl<'a, T> Iterator for KeyIterator<'a, T> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        if self.pos == self.map.string_data.len() {
            None
        } else {
            let length_string = decode_varint_slice(&self.map.string_data, &mut self.pos).unwrap();
            let text = unsafe {
                std::str::from_utf8_unchecked(
                    &self
                        .map
                        .string_data
                        .get_unchecked(self.pos..self.pos + length_string as usize),
                )
            };
            self.pos += length_string as usize;
            Some(text)
        }
    }
}

struct QuadraticProbing {
    hash: u32,
    i: u32,
    mask: u32,
}

impl QuadraticProbing {
    #[inline]
    fn compute(hash: u32, mask: u32) -> QuadraticProbing {
        QuadraticProbing { hash, i: 1, mask }
    }

    #[inline]
    fn next_probe(&mut self) -> u32 {
        self.i += 1;
        ((self.hash + (self.i + self.i * self.i)) >> 1) & self.mask
        // (self.hash + (self.i * self.i)) & self.mask
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_values_big() {
        use std::io::Read;

        let mut contents = String::new();
        std::fs::File::open("1342-0.txt")
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();

        let mut map = StringHashMap::<u32>::new();
        let mut counter = 0;
        for text in contents.split_whitespace() {
            let value = map.get_or_create(text, 0);
            *value += 1;
            counter += 1;
        }

        let sum: u32 = map.values().sum();
        assert_eq!(sum, counter);
        assert_eq!(map.string_data.len() < 1_000_000, true);

        dbg!(counter);

        // let num_one_time_probe= map.num_probes.iter().filter(|el| *el == &1).cloned().sum::<u32>();
        // let num_two_time_probe= map.num_probes.iter().filter(|el| *el == &2).cloned().sum::<u32>();
        // let num_more_than_one_time_probe= map.num_probes.iter().filter(|el| *el != &1).cloned().sum::<u32>();
        // dbg!(num_one_time_probe);
        // dbg!(num_two_time_probe);
        // dbg!(num_more_than_one_time_probe);
        // dbg!(map.existing);
    }
    #[test]
    fn values() {
        let mut hashmap = StringHashMap::<u32>::new();
        hashmap.get_or_create("blub", 1);

        let val: u32 = hashmap.values().sum();
        assert_eq!(val, 1);
    }
    #[test]
    fn simple() {
        let mut hashmap = StringHashMap::<u32>::new();
        let val = hashmap.get_or_create("blub1", 0);
        assert_eq!(*val, 0);
        *val += 1;

        let val = hashmap.get_or_create("blub2", 2);
        assert_eq!(*val, 2);
    }
    #[test]
    fn get_or_create() {
        let mut hashmap = StringHashMap::<u32>::new();
        let val = hashmap.get_or_create("blub", 0);
        assert_eq!(*val, 0);
        *val += 1;

        let val = hashmap.get_or_create("blub", 0);
        assert_eq!(*val, 1);
    }
    #[test]
    fn test_resize() {
        let mut hashmap = StringHashMap::<u32>::with_power_of_two_size(1);
        hashmap.get_or_create("blub1", 3);
        hashmap.get_or_create("blub2", 4);

        assert_eq!(hashmap.get_or_create("blub1", 3), &3);

        //should resize
        let val = hashmap.get_or_create("blub3", 5);
        assert_eq!(*val, 5);

        // // check values after resize
        assert_eq!(hashmap.get_or_create("blub1", 0), &3);
        assert_eq!(hashmap.get_or_create("blub2", 0), &4);
        assert_eq!(hashmap.get_or_create("blub3", 0), &5);
    }
    #[test]
    fn test_iter() {
        let mut hashmap = StringHashMap::<u32>::with_power_of_two_size(1);
        hashmap.get_or_create("blub1", 3);
        hashmap.get_or_create("blub2", 4);
        hashmap.get_or_create("blub3", 5);
        // // check values after resize
        assert_eq!(hashmap.get_or_create("blub1", 0), &3);
        assert_eq!(hashmap.get_or_create("blub2", 0), &4);
        assert_eq!(hashmap.get_or_create("blub3", 0), &5);

        assert_eq!(
            hashmap.keys().collect::<Vec<_>>(),
            &["blub1", "blub2", "blub3"]
        );
        assert_eq!(hashmap.values().collect::<Vec<_>>(), &[&5, &4, &3]);
        assert_eq!(hashmap.values_mut().collect::<Vec<_>>(), &[&5, &4, &3]);
        assert_eq!(
            hashmap.iter().collect::<Vec<_>>(),
            &[("blub3", &5), ("blub2", &4), ("blub1", &3),]
        );
        assert_eq!(
            hashmap.iter_mut().collect::<Vec<_>>(),
            &[("blub3", &mut 5), ("blub2", &mut 4), ("blub1", &mut 3),]
        );
    }

    #[test]
    fn test_get() {
        let mut hashmap = StringHashMap::<u32>::with_power_of_two_size(1);
        hashmap.get_or_create("blub1", 3);
        hashmap.get_or_create("blub2", 4);
        hashmap.get_or_create("blub3", 5);
        // // check values after resize
        assert_eq!(hashmap.get_or_create("blub1", 0), &3);
        assert_eq!(hashmap.get_or_create("blub2", 0), &4);
        assert_eq!(hashmap.get_mut("blub3"), Some(&mut 5));
        assert_eq!(hashmap.get("blub3"), Some(&5));
        assert_eq!(hashmap.get("blub1000"), None);
        assert_eq!(hashmap.get_mut("blub1000"), None);

        hashmap.shrink_to_fit();
    }
    #[test]
    fn test_len() {
        let mut hashmap = StringHashMap::<u32>::with_power_of_two_size(1);
        hashmap.get_or_create("blub1", 3);
        hashmap.get_or_create("blub2", 4);
        hashmap.get_or_create("blub3", 5);
        // // check values after resize
        assert_eq!(hashmap.len(), 3);
    }
}
