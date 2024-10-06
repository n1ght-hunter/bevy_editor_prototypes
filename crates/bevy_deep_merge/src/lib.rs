//! a trait for deep merging of two values of the same type
mod impls;

pub use bevy_deep_merge_derive::DeepMerge;

/// a trait for deep merging of two values of the same type
pub trait DeepMerge {
    /// deep merge two values of the same type
    /// other takes precedence over self
    fn deep_merge(&self, other: &Self) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deep_merge() {

        #[derive(Debug, PartialEq)]
        struct TestStruct {
            a: Option<i32>,
            b: Option<String>,
        }


        impl DeepMerge for TestStruct {
            fn deep_merge(&self, other: &Self) -> Self {
                Self {
                    a: self.a.deep_merge(&other.a),
                    b: self.b.deep_merge(&other.b),
                }
            }
        }

        let a = TestStruct {
            a: Some(1),
            b: Some("a".to_string()),
        };
        let b = TestStruct {
            a: Some(2),
            b: None,
        };
        let c = a.deep_merge(&b);
        assert_eq!(
            c,
            TestStruct {
                a: Some(2),
                b: Some("a".to_string())
            }
        );
    }

    #[test]
    fn test_deep_merge_derive() {
        #[derive(Debug, PartialEq, DeepMerge)]
        struct TestStruct {
            a: Option<i32>,
            b: Option<String>,
        }

        let a = TestStruct {
            a: Some(1),
            b: Some("a".to_string()),
        };
        let b = TestStruct {
            a: Some(2),
            b: None,
        };
        let c = a.deep_merge(&b);
        assert_eq!(
            c,
            TestStruct {
                a: Some(2),
                b: Some("a".to_string())
            }
        );
    }
}
