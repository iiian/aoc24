# Advent of Code '24 (Rust)

## Neat discoveries!

### Dec 24th:
  #### Generators in Rust (not the unstable nightly stuff)
  [genawaiter crate docs](https://docs.rs/genawaiter/latest/genawaiter/)
  ```rust
  let count_to_ten = gen!({
    for n in 0..10 {
        yield_!(n);
    }
  });
  ```

### Dec 16th:
  #### Get a modulus that is guaranteed to be positive:
  ```rust
  let x: i8 = 0;
  assert_eq!((x - 1).rem_euclid(4), 3);
  ```

### Dec 6th: 
  #### Take the cartesian product of two ranges:
  ```bash
  # 1.
  cargo add itertools
  ```


  ```rust
  // 2.
  let search_space = itertools::iproduct!(0..16, 0..64);

  for (x, y) in search_space { }
  ```