# connection-string-parser

A simple CLI application to parse connection strings.

**NOTE**: This is a CLI tool. Instructions to download it can be found on the [main
README](https://github.com/IgnisDa/rust-libs/blob/main/README.md).

## Usage

```bash
$ connection-string-parser $DATABASE_URL --part scheme
postgresql
```

## Acknowledgements

This library is just a CLI wrapper around the excellent [Url](https://docs.rs/url/) crate.
