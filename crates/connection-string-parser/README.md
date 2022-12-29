# connection-string-parser

A simple CLI application to parse connection strings.

## Usage

```bash
$ connection-string-parser $DATABASE_URL --part scheme
postgresql
```

## Acknowledgements

This library is just a CLI wrapper around the excellent [Url](https://docs.rs/url/) crate.
