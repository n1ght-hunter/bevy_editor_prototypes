use crate::DeepMerge;

/// Implement `DeepMerge` for type that implements `Clone`
/// can take multiple types that `Clone`
macro_rules! impl_deep_merge_for_raw_type {
    ($type:ty) => {
        impl DeepMerge for $type {
            fn deep_merge(&self, other: &Self) -> Self {
                other.clone()
            }
        }
    };

    ($($type:ty),*) => {
        $(impl_deep_merge_for_raw_type!($type);)*
    };
}

impl_deep_merge_for_raw_type!(bool);
impl_deep_merge_for_raw_type!(char);
impl_deep_merge_for_raw_type!(u8, u16, u32, u64, u128, usize);
impl_deep_merge_for_raw_type!(i8, i16, i32, i64, i128, isize);
impl_deep_merge_for_raw_type!(f32, f64);
impl_deep_merge_for_raw_type!(String);
impl_deep_merge_for_raw_type!(std::path::PathBuf);



impl <T> DeepMerge for Option<T> 
where T: DeepMerge + Clone
{
    fn deep_merge(&self, other: &Self) -> Self {
        match (self, other) {
            (Some(a), Some(b)) => Some(a.deep_merge(b)),
            (Some(a), None) => Some(a.clone()),
            (None, Some(b)) => Some(b.clone()),
            (None, None) => None,
        }
    }
}