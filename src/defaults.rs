/// Returns `true` if the `x` is equal to the default value of `T`.
///
/// ```rust
/// use sertools::is_default;
///
/// #[derive(serde::Serialize)]
/// struct Foo {
///     // Don't serialize zeroes.
///     #[serde(skip_serializing_if = "is_default")]
///     bar: usize
/// }
///
/// assert_eq!(serde_json::to_string(&Foo{bar: 0}).unwrap(), "{}");
/// assert_eq!(serde_json::to_string(&Foo{bar: 1}).unwrap(), "{\"bar\":1}");
/// ```
pub fn is_default<T: Default + PartialEq>(x: &T) -> bool {
    x == &T::default()
}
