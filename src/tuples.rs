/// Serialize `struct Typ(T)` as `T`.
///
/// ```rust
/// use sertools::transparent;
///
/// struct Foo(usize);
/// transparent!(Foo);
///
/// struct Bar {
///    n: usize,
/// }
/// transparent!(Bar, n);
/// ```
#[macro_export]
macro_rules! transparent {
    ($typ: ident) => {
        transparent!($typ, 0);
    };
    ($typ: ident, $field: tt) => {
        #[automatically_derived]
        impl serde::Serialize for $typ {
            /// Returns `serialize()` of the inner `T`.
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.$field.serialize(serializer)
            }
        }

        #[automatically_derived]
        impl<'de> serde::Deserialize<'de> for $typ {
            /// Returns `deserialize()` of the inner `T`.
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                serde::Deserialize::deserialize(deserializer).map(|v| Self { $field: v })
            }
        }
    };
}
