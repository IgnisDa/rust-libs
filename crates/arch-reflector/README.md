# Reflector

An implementation of the [Reflector](https://wiki.archlinux.org/title/reflector) tool in
Rust.

## Why?

One of the pain points of using reflector is that it is written in Python. This means that
you need to install Python and all of its dependencies just to use reflector. This is
especially painful on systems that don't have Python installed by default, such as the
docker [image](https://hub.docker.com/_/archlinux).

This implementation of reflector is written in Rust, so it can be downloaded as a single
binary and run without any dependencies.

## Installation

### Using [`eget`](https://github.com/zyedidia/eget)

```bash
eget IgnisDa/rust-libs --tag arch-reflector
```

### Manually

The binaries can be downloaded from
[here](https://github.com/IgnisDa/rust-libs/releases?q=arch-reflector&expanded=true).

## Usage

You should be able to get most things working with by just replacing the call to
the OG `reflector` with `arch-reflector`.

I will try to keep the interface as close to the original reflector as possible. Consult
the official [man page](https://man.archlinux.org/man/reflector.1) for more information.
You can also run the program with the `--help` flag to see the available options.
## Acknowledgements

The reflector project was developed by [Xyne](https://xyne.dev/) and all credits go to him.
