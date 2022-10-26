# mccs

[![release-badge][]][cargo] [![docs-badge][]][docs] [![license-badge][]][license]

`mccs` implements the VESA [Monitor Control Command Set](https://en.wikipedia.org/wiki/Monitor_Control_Command_Set).
The library is split up into a few sub-crates:

- [`mccs`](https://crates.io/crates/mccs) contains the common types describing the MCCS and VCP data structures.
- [`mccs-caps`](https://crates.io/crates/mccs-caps) provides a parser for the MCCS capability string.
- [`mccs-db`](https://crates.io/crates/mccs-db) contains the human-readable descriptions of VCP features from the
  MCCS spec.

## [Documentation][docs]

See the [documentation][docs] for up to date information.

[release-badge]: https://img.shields.io/crates/v/mccs.svg?style=flat-square
[cargo]: https://crates.io/crates/mccs
[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: http://arcnmx.github.io/mccs-rs/mccs/
[license-badge]: https://img.shields.io/badge/license-MIT-ff69b4.svg?style=flat-square
[license]: https://github.com/arcnmx/mccs-rs/blob/master/COPYING
