# swarmd

<p align="center">
  <a href="https://swarmd.io">
    <img src="https://swarmd.io/swarmd.svg" height="128" width="128">
    <h3 align="center"><a href="https://swarmd.io">Swarmd Preview</a></h3>
    <h4 align="center"><a href="https://docs.swarmd.io">Documentation</a></h4>
  </a>
</p>

----
> [!NOTE]
> Swarmd is still in preview and not every components are yet open sourced. This
is the CLI which is used to interact with Swarmd.


- [`swarmd`](./cli/README.md) is the CLI tool desgined to intereact with [swarmd
workers](https://swarmd.io).

## Installation
---- 

### Install with Cargo

```bash
cargo install swarmd --locked
```

## Getting started
----

Once you have swarmd, it'll be quite easy for you to deploy your first Swarmd worker!

### Create your Swarmd Worker

```bash
swarmd login
swarmd create --template typescript demo_worker
```

### Configure it

### Deploy it!

```bash
swarmd deploy
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

