# Bevy Reflect Deep Merge

This crate provides functions to merge two copys of the same type.

## Example

```rust
use bevy_reflect_deepmerge::deep_merge;

#[derive(Debug, PartialEq, Reflect, Default)]
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

let struct_3 = deep_merge(&struct_1, &struct_2);

assert_eq!(
    struct_3,
    TestStruct {
        a: Some(2),
        b: Some("a".to_string())
    }
);
```