# Visen

A screenplay visualization engine.

## ⚠️ WARNING ⚠️

This tool is under active development and there will regularly be breaking changes without notice.

## Dependencies

* [Git](https://git-scm.com/downloads)
* [Rust](https://www.rust-lang.org/tools/install)

## Installation

Install dependencies 👆.

Clone the repository.

```bash
git clone https://github.com/matthewnoel/visen.git
```

Navigate to the directory.

```bash
cd visen
```

Build and install the tool.

```bash
cargo install --path .
```

## Usage

```bash
# Creates and populates a new project called my-movie
visen init my-movie
```

```bash
# Parses the local SCRIPT.md file, prints data about the script to the console, updates the README.md file, and updates the HTML version of the script in the docs directory.
visen
```
