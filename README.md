# quantified

![CI status](https://github.com/nhynes/quantified/workflows/CI/badge.svg)(https://github.com/nhynes/quantified/actions?query=workflow%3ACI)
[![quantified on docs.rs](https://docs.rs/quantified/badge.svg)](https://docs.rs/quantified)

_Something, Everything, and Nothing for Everyone._

```rust
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Quantified<T> {
    None,
    Some(T),
    Excluding(T),
    All,
}
```
