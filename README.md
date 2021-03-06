
## Inohashmap

Stores values for strings in a Hashmap in a fast and compact way.

Good to count strings and assign ids to them or similar. Address space of string data is limited to u32::MAX (4GB).
string data is size in bytes of all uniquely inserted strings + string length metadata per string.

### Example

```
use inohashmap::StringHashMap;
let mut hashmap = StringHashMap::<u32>::new();
let val = hashmap.get_or_create("blub1", 0);
assert_eq!(*val, 0);
*val += 1;
let val = hashmap.get_or_create("blub2", 2);
assert_eq!(*val, 2);
```

### Memory Consumption
Memory Consumption is lower than with a regular hashmap, 30% lower in the [compare_allocations](compare_allocations/README.md) test.


### Bench

```
running 11 tests
test tests::bench_fnv                            ... bench:     141,906 ns/iter (+/- 9,414)
test tests::bench_fnv_full                       ... bench:   5,180,066 ns/iter (+/- 392,488)
test tests::bench_fnv_full_get                   ... bench:   3,914,865 ns/iter (+/- 210,328)
test tests::bench_hasmap                         ... bench:     124,689 ns/iter (+/- 5,389)
test tests::bench_hasmap_full                    ... bench:   5,006,276 ns/iter (+/- 166,848)
test tests::bench_hasmap_full_get                ... bench:   4,015,903 ns/iter (+/- 149,086)
test tests::bench_hasmap_full_large_struct       ... bench:   5,533,528 ns/iter (+/- 201,944)
test tests::bench_tant_termmap                   ... bench:     123,695 ns/iter (+/- 6,106)
test tests::bench_tant_termmap_full              ... bench:   5,454,897 ns/iter (+/- 157,208)
test tests::bench_tant_termmap_full_get          ... bench:   5,047,846 ns/iter (+/- 181,768)
test tests::bench_tant_termmap_full_large_struct ... bench:   5,644,069 ns/iter (+/- 269,745)
```

