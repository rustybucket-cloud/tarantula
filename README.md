# Tarantula

Tarantula lets you run web apps like desktop applications. It was inspired by the web app experience on [Omarchy](https://github.com/basecamp/omarchy).

Tarantula allows you to open the web app from your application launcher and from the command line.

Tarantula requires a Chromium-based browser (Chrome, Edge, Brave, Arc, Vivaldi) to be installed on your system.

## Table of Contents

- [Installation](##installation)
- [Usage](##usage)

## Installation

### Cargo

```bash
cargo install tarantula
```

### Build from source

```bash
git clone https://github.com/rustbucket-cloud/tarantula.git
cd tarantula
cargo build --release
cp target/release/tarantula /usr/local/bin/
```

### AUR

(Coming soon)

## Usage

Run a web app:

```bash
tarantula <name>
```

Add a web app:

```bash
tarantula install <name> <url>
```

Remove a web app:

```bash
tarantula uninstall <name>
```

List installed web apps:

```bash
tarantula list
```

Update a web app:

```bash
tarantula update <name>
```
