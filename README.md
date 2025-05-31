Like `Cow`, but `T` is covariant

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
