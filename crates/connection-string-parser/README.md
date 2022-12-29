# connection-string-parser

A simple CLI application to parse connection strings.

## Installation

### Using [`eget`](https://github.com/zyedidia/eget)

```bash
eget IgnisDa/rust-libs --tag connection-string-parser
```

### Manually

The binaries can be downloaded from
[here](https://github.com/IgnisDa/rust-libs/releases?q=connection-string-parser&expanded=true).

## Usage

```bash
$ connection-string-parser $DATABASE_URL --part scheme
postgresql
```

## Acknowledgements

This library is just a CLI wrapper around the excellent [Url](https://docs.rs/url/) crate.
