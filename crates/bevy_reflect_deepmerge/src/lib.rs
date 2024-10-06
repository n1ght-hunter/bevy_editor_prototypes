//! Deep merge for Bevy Reflect.

use bevy::reflect::{
    Array, DynamicList, DynamicMap, Enum, GetField, GetTupleField, List, Map, PartialReflect,
    Reflect, ReflectMut, ReflectRef, Set, Struct, Tuple, TupleStruct, TypeData,
};

fn deep_merge<T: Reflect + Default>(a: T, b: T) -> T {
    // Create a new instance of T
    let mut result = T::default();

    // Get reflected references to a, b, and result
    let a_ref = a.reflect_ref();
    let b_ref = b.reflect_ref();
    let mut result_ref = result.reflect_mut();

    merge_inner(Some(a_ref), Some(b_ref), &mut result_ref);

    // Return the merged result
    result
}

fn merge_inner(a: Option<ReflectRef>, b: Option<ReflectRef>, output: &mut ReflectMut) {
    // Merge fields
    match (a, b, output) {
        (
            Some(ReflectRef::Struct(a_struct)),
            Some(ReflectRef::Struct(b_struct)),
            ReflectMut::Struct(ouput),
        ) => {
            merge_struct(a_struct, b_struct, ouput);
        }
        (
            Some(ReflectRef::Value(_)),
            Some(ReflectRef::Value(b_value)),
            ReflectMut::Value(output),
        ) => {
            output.apply(b_value);
        }
        (None, Some(ReflectRef::Value(b_value)), ReflectMut::Value(output)) => {
            output.apply(b_value);
        }
        (Some(ReflectRef::Value(a_value)), None, ReflectMut::Value(output)) => {
            output.apply(a_value);
        }
        (
            Some(ReflectRef::Enum(a_enum)),
            Some(ReflectRef::Enum(b_enum)),
            ReflectMut::Enum(output),
        ) => {
            merge_enum(a_enum, b_enum, output);
        }
        (
            Some(ReflectRef::List(a_list)),
            Some(ReflectRef::List(b_list)),
            ReflectMut::List(output),
        ) => {
            merge_list(a_list, b_list, output);
        }

        (
            Some(ReflectRef::Array(a_array)),
            Some(ReflectRef::Array(b_array)),
            ReflectMut::Array(output),
        ) => {
            merge_array(a_array, b_array, output);
        }

        (Some(ReflectRef::Map(a_map)), Some(ReflectRef::Map(b_map)), ReflectMut::Map(output)) => {
            merge_map(a_map, b_map, output);
        }

        (Some(ReflectRef::Set(a_set)), Some(ReflectRef::Set(b_set)), ReflectMut::Set(output)) => {
            merge_set(a_set, b_set, output);
        }

        (
            Some(ReflectRef::TupleStruct(a_struct)),
            Some(ReflectRef::TupleStruct(b_struct)),
            ReflectMut::TupleStruct(output),
        ) => {
            merge_tuble_struct(a_struct, b_struct, output);
        }

        (
            Some(ReflectRef::Tuple(a_tuple)),
            Some(ReflectRef::Tuple(b_tuple)),
            ReflectMut::Tuple(output),
        ) => {
            merge_tuple(a_tuple, b_tuple, output);
        }
        (None, None, _) => {
            // Do nothing
        }
        (a, b, _) => {
            panic!(
                "Cannot merge {:?} and {:?}",
                a.map(|a| a.kind()),
                b.map(|b| b.kind())
            );
        }
    }
}

fn merge_tuble_struct(
    a_struct: &dyn TupleStruct,
    b_struct: &dyn TupleStruct,
    ouput: &mut &mut dyn TupleStruct,
) {
    for (i, field) in a_struct.iter_fields().enumerate() {
        let a = field.reflect_ref();
        let b = b_struct.field(i).unwrap().reflect_ref();
        let mut output = ouput.field_mut(i).unwrap().reflect_mut();
        merge_inner(Some(a), Some(b), &mut output);
    }
}

fn merge_tuple(a_tuple: &dyn Tuple, b_tuple: &dyn Tuple, ouput: &mut &mut dyn Tuple) {
    for (i, field) in a_tuple.iter_fields().enumerate() {
        let a = field.reflect_ref();
        let b = b_tuple.field(i).unwrap().reflect_ref();
        let mut output = ouput.field_mut(i).unwrap().reflect_mut();
        merge_inner(Some(a), Some(b), &mut output);
    }
}

fn merge_set(a_set: &dyn Set, b_set: &dyn Set, ouput: &mut &mut dyn Set) {
    match (a_set.len(), b_set.len()) {
        (0, 0) => {
            // Do nothing
        }
        (0, _) => {
            ouput.apply(b_set.as_partial_reflect());
        }
        (_, 0) => {
            ouput.apply(a_set.as_partial_reflect());
        }
        _ => {
            ouput.apply(b_set.as_partial_reflect());
        }
    }
}

fn merge_map(a_map: &dyn Map, b_map: &dyn Map, ouput: &mut &mut dyn Map) {
    match (a_map.len(), b_map.len()) {
        (0, 0) => {
            // Do nothing
        }
        (0, _) => {
            ouput.apply(b_map.as_partial_reflect());
        }
        (_, 0) => {
            ouput.apply(a_map.as_partial_reflect());
        }
        _ => {
            ouput.apply(b_map.as_partial_reflect());
        }
    }
}

fn merge_array(a: &dyn Array, b: &dyn Array, ouput: &mut &mut dyn Array) {
    match (a.len(), b.len()) {
        (0, 0) => {
            // Do nothing
        }
        (0, _) => {
            ouput.apply(b.as_partial_reflect());
        }
        (_, 0) => {
            ouput.apply(a.as_partial_reflect());
        }
        _ => {
            ouput.apply(b.as_partial_reflect());
        }
    }
}

fn merge_list(a_list: &dyn List, b_list: &dyn List, ouput: &mut &mut dyn List) {
    match (a_list.len(), b_list.len()) {
        (0, 0) => {
            // Do nothing
        }
        (0, _) => {
            ouput.apply(b_list.as_partial_reflect());
        }
        (_, 0) => {
            ouput.apply(a_list.as_partial_reflect());
        }
        _ => {
            ouput.apply(b_list.as_partial_reflect());
        }
    }
}

fn merge_struct(a_struct: &dyn Struct, b_struct: &dyn Struct, ouput: &mut &mut dyn Struct) {
    for (i, field) in a_struct.iter_fields().enumerate() {
        let field_name = a_struct.name_at(i).unwrap();
        let b_field = b_struct.field(field_name).unwrap();
        let a = field.reflect_ref();
        let b = b_field.reflect_ref();
        let mut output = ouput.field_at_mut(i).unwrap().reflect_mut();
        merge_inner(Some(a), Some(b), &mut output);
    }
}

fn merge_enum(a_enum: &dyn Enum, b_enum: &dyn Enum, ouput: &mut &mut dyn Enum) {
    let a_variant = a_enum.variant_type();
    let b_variant = b_enum.variant_type();

    match a_variant {
        bevy::reflect::VariantType::Struct => {
            ouput.apply(b_enum.as_partial_reflect());
            // If the variants are different, use the value from b
            if a_variant != b_variant {
                return;
            }
            for (i, field) in a_enum.iter_fields().enumerate() {
                let a = field.value().reflect_ref();
                let b = b_enum.field_at(i).unwrap().reflect_ref();
                let mut output = ouput.field_at_mut(i).unwrap().reflect_mut();
                merge_inner(Some(a), Some(b), &mut output);
            }
        }
        bevy::reflect::VariantType::Tuple => {
            match (a_enum.reflect_type_ident(), b_variant) {
                (Some("Option"), bevy::reflect::VariantType::Unit) => {
                    ouput.apply(a_enum.as_partial_reflect());
                }
                _ => {
                    ouput.apply(b_enum.as_partial_reflect());
                    // If the variants are different, use the value from b
                    if a_variant != b_variant {
                        return;
                    }
                    for (i, field) in a_enum.iter_fields().enumerate() {
                        let a = field.value().reflect_ref();
                        let b = b_enum.field_at(i).unwrap().reflect_ref();
                        let mut output = ouput.field_at_mut(i).unwrap().reflect_mut();
                        merge_inner(Some(a), Some(b), &mut output);
                    }
                }
            }
        }
        bevy::reflect::VariantType::Unit => {
            ouput.apply(b_enum.as_partial_reflect());
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::utils::HashMap;

    use super::*;

    #[derive(Debug, PartialEq, Reflect, Default)]
    struct TestStruct {
        a: Option<i32>,
        b: Option<String>,
        c: Option<bool>,
    }

    #[test]
    fn test_deep_merge() {
        let a = TestStruct {
            a: Some(1),
            b: Some("hello".to_string()),
            c: Some(true),
        };
        let b = TestStruct {
            a: None,
            b: Some("world".to_string()),
            c: Some(false),
        };
        let result = deep_merge(a, b);

        assert_eq!(
            result,
            TestStruct {
                a: Some(1),
                b: Some("world".to_string()),
                c: Some(false),
            },
        );
    }

    #[derive(Debug, PartialEq, Reflect, Default)]
    struct ComplexType {
        a: TestStruct,
        b: Vec<TestStruct>,
        c: HashMap<String, TestStruct>,
        d: Option<TestStruct>,
    }

    #[test]
    fn test_deep_merge_complex() {
        let a = ComplexType {
            a: TestStruct {
                a: Some(1),
                b: Some("hello".to_string()),
                c: Some(true),
            },
            b: vec![
                TestStruct {
                    a: Some(1),
                    b: Some("hello".to_string()),
                    c: Some(true),
                },
                TestStruct {
                    a: Some(1),
                    b: Some("hello".to_string()),
                    c: Some(true),
                },
            ],
            c: {
                let mut map = HashMap::new();
                map.insert(
                    "key".to_string(),
                    TestStruct {
                        a: Some(1),
                        b: Some("hello".to_string()),
                        c: Some(true),
                    },
                );
                map
            },
            d: Some(TestStruct {
                a: Some(1),
                b: Some("hello".to_string()),
                c: Some(true),
            }),
        };
        let b = ComplexType {
            a: TestStruct {
                a: None,
                b: Some("world".to_string()),
                c: Some(false),
            },
            b: vec![
                TestStruct {
                    a: Some(1),
                    b: Some("world".to_string()),
                    c: Some(true),
                },
                TestStruct {
                    a: Some(1),
                    b: Some("world".to_string()),
                    c: Some(true),
                },
            ],
            c: {
                let mut map = HashMap::new();
                map.insert(
                    "key".to_string(),
                    TestStruct {
                        a: Some(1),
                        b: Some("hello".to_string()),
                        c: Some(true),
                    },
                );
                map
            },
            d: None,
        };
        let result = deep_merge(a, b);

        assert_eq!(
            result,
            ComplexType {
                a: TestStruct {
                    a: Some(1),
                    b: Some("world".to_string()),
                    c: Some(false),
                },
                b: vec![
                    TestStruct {
                        a: Some(1),
                        b: Some("world".to_string()),
                        c: Some(true),
                    },
                    TestStruct {
                        a: Some(1),
                        b: Some("world".to_string()),
                        c: Some(true),
                    },
                ],
                c: {
                    let mut map = HashMap::new();
                    map.insert(
                        "key".to_string(),
                        TestStruct {
                            a: Some(1),
                            b: Some("hello".to_string()),
                            c: Some(true),
                        },
                    );
                    map
                },
                d: Some(TestStruct {
                    a: Some(1),
                    b: Some("hello".to_string()),
                    c: Some(true),
                }),
            },
        );
    }
}
