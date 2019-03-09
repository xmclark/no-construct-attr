# no-construct-attr

Prevent a struct from being instantiated. Will error if the attribute is put on something that is not a struct.

```rust
#[no_construct]
struct Foo;

fn make_foo() -> Foo {
  Foo // error!
}
```

## Motivation

I had this idea at work one day, and it became an excuse to learn the basics of procedural macros.
