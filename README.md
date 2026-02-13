# Meu

[![PyPI - Version](https://img.shields.io/pypi/v/meu?style=for-the-badge)](https://pypi.org/project/meu/)
[![License](https://img.shields.io/github/license/campbellmbrown/meu?style=for-the-badge)](LICENSE)
[![Deploy](https://img.shields.io/github/actions/workflow/status/campbellmbrown/meu/deploy.yaml?style=for-the-badge&logo=github&label=Deploy)](https://github.com/campbellmbrown/meu/actions/workflows/deploy.yaml)
[![Develop](https://img.shields.io/github/actions/workflow/status/campbellmbrown/meu/develop.yaml?branch=main&style=for-the-badge&logo=github&label=Develop)](https://github.com/campbellmbrown/meu/actions/workflows/develop.yaml)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

## Overview

Meu is a set of convenient command-line tools for everyday tasks, written in Rust.

## Installation

Meu is available on [PyPI](https://pypi.org/project/meu/).

You can install it using your preferred Python package manager. For example, using `uv`:

```bash
uv tool install meu
```

To upgrade:

```bash
uv tool update meu
```

## Usage

### `scan`

Scan for pattern matches in files using either a single file or files in a directory.

To scan a single file:

```bash
meu scan --file <FILE> --pattern <PATTERN> [--first-match]
```

* `--file <FILE>`: The file to scan
* `--pattern <PATTERN>`: The pattern to search for (supports regular expressions)
* `--first-match`: Stop on first match in the file

To scan files in a directory:

```bash
meu scan --dir <DIR> --pattern <PATTERN> [--glob <GLOB>] [--first-match]
```

* `--dir <DIR>`: The directory to scan
* `--pattern <PATTERN>`: The pattern to search for (supports regular expressions)
* `--glob <GLOB>`: The glob pattern to match files (e.g. `**/*.txt`). Default is `*`
* `--first-match`: Stop on first match in any file

## Contributing

Contributions are welcome! Please see the [contributing guide](CONTRIBUTING.md) for details.
