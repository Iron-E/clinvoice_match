# Winvoice Match

<!-- cargo-rdme start -->

`winvoice_match` contains types that have counterparts with identical layout in [`winvoice_schema`]. The only difference between the structures in this crate and [`winvoice_schema`] is that the types in this crate can be used to describe any number of their counterpart types.

The ability to "describe" other types comes from [`Match`], [`MatchSet`], and [`MatchStr`]. As this is the distinguishing feature of the crate, none of those three types have equivalents in [`winvoice_schema`].

## Features

* `serde` adds support for the [`serde`] crate. This crate is intended for and tested with [`serde_yaml`](https://docs.serde.rs/serde_yaml/) in particular.

## Re-exports

This crate re-exports [`humantime_serde::Serde`], as it is required to deserialize the `increment` of a [`MatchJob`] via human-readable time (e.g. "15min").

## Examples

The following [`MatchEmployee`] represents all [`Employee`](winvoice_schema::Employee)s who meet all of the following criteria:

* Have a `name` starting with 'A', 'B', or 'C'.
* Have a `status` equal to "Hired".
* Have a `title` not equal to "CEO".

```rust
use winvoice_match::{Match, MatchEmployee, MatchStr};

let _ = MatchEmployee {
  name: MatchStr::Regex("^[ABC]".into()),
  status: "Hired".to_owned().into(),
  title: MatchStr::Not(Box::new("CEO".to_string().into())),
  ..Default::default()
};
```

<!-- cargo-rdme end -->
