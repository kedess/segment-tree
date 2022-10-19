## segment-tree
This is a data structure that allows, with O(log(n)) asymptotics, to implement any operations defined on a set on which this operation is associative and there is a neutral element with respect to this operation, that is, on a monoid. For example, summing on a set of natural numbers, searching for a minimum or maximum on any set of numbers.

<b>Note that both boundaries are included in queries</b>

### Usage example:
```rust
use segment_tree::RSQ;

fn main() {
    let values = [1, 2, 3, 4, 5];
    let rsq = RSQ::build(&values).unwrap();

    assert_eq!(rsq.sum(0, 4).unwrap(), 15);
    assert_eq!(rsq.sum(1, 4).unwrap(), 14);
    assert_eq!(rsq.sum(1, 2).unwrap(), 5);
    assert_eq!(rsq.sum(1, 1).unwrap(), 2);

    // Update element
    rsq.upddate(2, 6).unwrap();
    assert_eq!(rsq.sum(0, 4).unwrap(), 18);
    assert_eq!(rsq.sum(2, 4).unwrap(), 15);
    assert_eq!(rsq.sum(3, 4).unwrap(), 9);
}
```

```rust
use segment_tree::RMaxQ;

fn main() {
    let values = [1, 2, 3, 4, 5];
    let mut rmq = RMaxQ::build(&values).unwrap();

    assert_eq!(rmq.max(0, 4).unwrap(), 5);
    assert_eq!(rmq.max(1, 4).unwrap(), 5);
    assert_eq!(rmq.max(1, 2).unwrap(), 3);
    assert_eq!(rmq.max(1, 1).unwrap(), 2);

    rmq.upddate(2, 6).unwrap();
    assert_eq!(rmq.max(0, 4).unwrap(), 6);
    assert_eq!(rmq.max(2, 4).unwrap(), 6);
    assert_eq!(rmq.max(3, 4).unwrap(), 5);
}
```

### Cargo.toml
```bash
[dependencies]
segment-tree = {git = "https://github.com/mingendo/segment-tree.git", branch="main"}
```
