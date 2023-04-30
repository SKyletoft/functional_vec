# Functional Vec

Owning versions of all mutable vec methods. Makes vecs slightly more
ergonomic to use in folds.

## Example

```rust
use functional_vec::FunctionalVec;

let v = (0..10).fold(Vec::new(), |acc, curr| acc.push_new(curr));
assert_eq!(v, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
```
