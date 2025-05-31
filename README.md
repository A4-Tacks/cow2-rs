Like `Cow`, but `T` is covariant

Implemented similar to [rust#96312]

Since `std::borrow::Cow` uses associated type, it invariant,
see [rust#115799] for details

[rust#96312]: https://github.com/rust-lang/rust/pull/96312
[rust#115799]: https://github.com/rust-lang/rust/issues/115799

# Examples

**Use `std::borrow::Cow` compile-fail case**:

```rust,compile_fail
use std::borrow::Cow;

fn foo<'a>(_: Cow<'a, [Cow<'a, str>]>) { }
fn bar<'a>(x: Cow<'a, [Cow<'static, str>]>) {
    foo(x);
}
```

**Use `cow2::Cow` compile-passed case**:


```rust
use cow2::Cow;

fn foo<'a>(_: Cow<'a, [Cow<'a, str>]>) { }
fn bar<'a>(x: Cow<'a, [Cow<'static, str>]>) {
    foo(x);
}
```
