# swarmd

<p align="center">
  <a href="https://swarmd.io">
    <img src="https://swarmd.io/swarmd.svg" height="128" width="128">
    <h3 align="center"><a href="https://swarmd.io">Swarmd Preview</a></h3>
  </a>
</p>

----

**Warning**: Swarmd is still in preview for now, if you want to have early
access, feel free to drop me a mail at `anthony@swarmd.io`.

This repository host everything related [Swarmd Workers](https://swarmd.io).

- [`swarmd`](./cli/README.md) is the CLI tool desgined to intereact with [swarmd
workers](https://swarmd.io).

## Installation
---- 

### Install with Cargo

```bash
cargo install swarmd --locked
```

## Development

You can build the development version in the repo by running

```bash
cargo install --path cli/.
```

### Cargo, build it from source

```bash
cargo install swarmd
```

## Release

We do use [release-plz](https://github.com/MarcoIeni/release-plz) to handle
release.

