# Bevy Deep Merge

This crate provides functions to merge two copys of the same type.

## Example

```rust
use bevy_deep_merge::DeepMerge;

#[derive(Debug, PartialEq, DeepMerge)]
struct TestStruct {
    a: Option<i32>,
    b: Option<String>,
}

let struct_1 = TestStruct {
    a: Some(1),
    b: Some("a".to_string()),
};
let struct_2 = TestStruct {
    a: Some(2),
    b: None,
};
let struct_3 = struct_1.deep_merge(&struct_2);
assert_eq!(
    c,
    TestStruct {
        a: Some(2),
        b: Some("a".to_string())
    }
);
```