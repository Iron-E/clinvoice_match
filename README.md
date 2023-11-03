# Winvoice Match

<!-- cargo-rdme start -->

[`winvoice_match`](https://docs.rs/winvoice-match/latest/winvoice_match/) contains counterpart types to those in [`winvoice_schema`], the difference being that
these types are built upon *descriptors* rather than concrete values. For example, the following [`MatchEmployee`]
represents all [`Employee`](winvoice_schema::Employee)s who:

* Have a `name` starting with 'A', 'B', or 'C'.
* Have a `status` equal to "Hired".
* Have a `title` not equal to "CEO".

```rust
use winvoice_match::{Match, MatchEmployee, MatchStr};

let _ = MatchEmployee {
  department: "Executive".to_owned().into(),
  name: MatchStr::Regex("^[ABC]".into()),
  title: MatchStr::Not(Box::new("CEO".to_owned().into())),
  ..Default::default()
};
```

The ability to "describe" values comes from [`Match`], [`MatchOption`], [`MatchSet`], and [`MatchStr`].

## Features

* `serde` adds support for the [`serde`] crate. This crate is tested using
  [`serde_json`](https://docs.serde.rs/serde_json/) and [`serde_yaml`](https://docs.serde.rs/serde_yaml/).

## Re-exports

This crate re-exports [`humantime_serde::Serde`], as it is required to deserialize the
`increment` of a [`MatchJob`] via human-readable time (e.g. "15min").

<!-- cargo-rdme end -->
