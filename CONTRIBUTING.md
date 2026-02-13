# Contributing to meu

## Prerequisites

Meu is written in Rust. You'll need to install the [Rust toolchain](https://www.rust-lang.org/tools/install) to build and contribute.

### Code Quality

To lint Rust code, run:

```bash
cargo clippy --fix --allow-dirty -- -W clippy::pedantic
```

To format Rust code, run:

```bash
cargo fmt
```

### Unit Tests

To run unit tests, run:

```bash
cargo test
```

### Publishing

Publishing is done using [`maturin`](https://www.maturin.rs/) automatically in the [deploy.yaml](.github/workflows/deploy.yaml) GitHub Actions workflow.
This workflow is triggered when a new tag is pushed to the repository.

### Versioning

Meu uses [Semantic Versioning](https://semver.org/).
