use quote::quote;
use syn::{parse_macro_input, Item};
use syn::export::TokenStream;

/// Successful Usage:
/// ```
/// use no_construct_attr::no_construct;
///
/// #[no_construct]
/// struct Foo;
///
/// #[no_construct]
/// struct Bar {
///   foo: Foo,
/// }
/// #[no_construct]
/// struct Baz(Bar);
/// ```
/// ```compile_fail
/// use no_construct_attr::no_construct;
/// fn make_stuff() {
///   // Attempting to constructing the types with curly braces will fail.
///   Foo{};
/// }
/// ```
#[proc_macro_attribute]
pub fn no_construct(_: TokenStream, input: TokenStream) -> TokenStream {
    if let Item::Struct(is) = parse_macro_input!(input as Item) {
        let name = is.ident;
        TokenStream::from(quote!(struct #name {_private: ()}))
    }
    else {
        panic!("Must call #[no_construct] on a `struct`.");
    }
}
