
### Fixed Length Test branch

Experiment if strings are all 8bytes long and it is know up front. A lot of memory can be saved. Performance is unaffected.

```
dhat: Total:     941,760 bytes in 140 blocks
dhat: At t-gmax: 65,536 bytes in 3 blocks
dhat: At t-end:  0 bytes in 0 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html

dhat: Total:     2,180,160 bytes in 19,060 blocks
dhat: At t-gmax: 217,128 bytes in 1,795 blocks
dhat: At t-end:  0 bytes in 0 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.htm

dhat: Total:     3,204,160 bytes in 80 blocks
dhat: At t-gmax: 312,096 bytes in 6 blocks
dhat: At t-end:  0 bytes in 0 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html

```



Compares allocations between inohashmap, tantivity internal used hashmap and fnv

```
cargo run --bin inohash 
cargo run --bin tant_termmap
cargo run --bin fnv

```

### DHAT Profile
```
➜  compare_allocations git:(main) ✗ cargo run --bin inohash     
Compiling inohashmap v0.1.0 (/home/pascal/Development/insert_only_string_hash_map)
Compiling compare_allocations v0.1.0 (/home/pascal/Development/insert_only_string_hash_map/compare_allocations)
    Finished dev [unoptimized + debuginfo] target(s) in 0.69s
     Running `target/debug/inohash`
dhat: Total:     7,813,120 bytes in 150 blocks
dhat: At t-gmax: 524,288 bytes in 3 blocks
dhat: At t-end:  0 bytes in 0 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
➜  compare_allocations git:(main) ✗ cargo run --bin tant_termmap
   Compiling compare_allocations v0.1.0 (/home/pascal/Development/insert_only_string_hash_map/compare_allocations)
    Finished dev [unoptimized + debuginfo] target(s) in 0.69s
     Running `target/debug/tant_termmap`
dhat: Total:     7,863,360 bytes in 140 blocks
dhat: At t-gmax: 661,536 bytes in 6 blocks
dhat: At t-end:  0 bytes in 0 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
➜  compare_allocations git:(main) ✗ cargo run --bin fnv         
   Compiling compare_allocations v0.1.0 (/home/pascal/Development/insert_only_string_hash_map/compare_allocations)
    Finished dev [unoptimized + debuginfo] target(s) in 0.65s
     Running `target/debug/fnv`
dhat: Total:     11,219,930 bytes in 137,850 blocks
dhat: At t-gmax: 865,262 bytes in 7,171 blocks
dhat: At t-end:  0 bytes in 0 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html

```
