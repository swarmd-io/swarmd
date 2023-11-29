# swarmd

<p align="center">
  <a href="https://swarmd.io">
    <img src="https://swarmd.io/swarmd.svg" height="128" width="128">
    <h3 align="center">Swarmd Preview</h3>
  </a>
</p>

----

**Warning**: Swarmd is still in preview for now, if you want to have early
access, feel free to drop me a mail at `anthony@swarmd.io`.

This repository host everything related Swarmd Workers.

- [`swarmd`](./cli/README.md) is the CLI tool desgined to intereact with [swarmd
workers](https://swarmd.io).

## Release

We do use [release-plz](https://github.com/MarcoIeni/release-plz) to handle
release.

## Installation
---- 

### Development

You can build the development version in the repo by running

```bash
cargo install --path cli/.
```

### Cargo, build it from source

```bash
cargo install swarmd
```
