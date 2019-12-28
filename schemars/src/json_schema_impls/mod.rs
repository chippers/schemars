macro_rules! no_ref_schema {
    () => {
        fn is_referenceable() -> bool {
            false
        }
    };
}

macro_rules! forward_impl {
    (($($impl:tt)+) => $target:ty) => {
        impl $($impl)+ {
            fn is_referenceable() -> bool {
                <$target>::is_referenceable()
            }

            fn schema_name() -> String {
                <$target>::schema_name()
            }

            fn json_schema(gen: &mut SchemaGenerator) -> Schema {
                <$target>::json_schema(gen)
            }

            fn json_schema_optional(gen: &mut SchemaGenerator) -> Schema {
                <$target>::json_schema_optional(gen)
            }
        }
    };
    ($ty:ty => $target:ty) => {
        forward_impl!((JsonSchema for $ty) => $target);
    };
}

mod array;
#[cfg(std_atomic)]
mod atomic;
#[cfg(feature = "chrono")]
mod chrono;
#[cfg(feature = "indexmap")]
mod indexmap;
#[cfg(feature = "either")]
mod either;
#[cfg(feature = "uuid")]
mod uuid;
#[cfg(feature = "smallvec")]
mod smallvec;
#[cfg(feature = "arrayvec")]
mod arrayvec;
mod core;
mod ffi;
mod maps;
#[cfg(num_nonzero_signed)]
mod nonzero_signed;
mod nonzero_unsigned;
mod primitives;
mod sequences;
mod serdejson;
mod time;
mod tuple;
mod wrapper;
