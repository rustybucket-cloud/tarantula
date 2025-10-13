# Tarantula

Tarantula lets you run web apps like desktop applications. It was inspired by the web app experience on [Omarchy](https://github.com/basecamp/omarchy).

Open web apps from your application launcher and from the command line.

A Chromium-based browser is required (Chrome, Edge, Brave, Arc, Vivaldi) to be installed on your system.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
  - [Run an app](#run-an-app)
  - [Add an app](#add-an-app)
  - [Remove an app](#remove-an-app)
  - [List installed web apps](#list-installed-web-apps)
  - [Update an app](#update-an-app)
  - [Selecting a browser](#selecting-a-browser)

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

Open tarantula UI (Coming Soon):

```bash
tarantula
```

### Run an app

Tarantula creates a `.desktop` file for your installed web apps.
You should be able to run your web apps from your application launcher.

You can also run an app from the command line:
```bash
tarantula <name>
```

### Add an app

```bash
tarantula install <name> <url>
```

### Remove an app

```bash
tarantula uninstall <name>
```

### List installed web apps

```bash
tarantula list
```

### Update an app

```bash
tarantula update <name>
```

### Selecting a browser

By default Tarantula will use your system's default browser.

To configure Tarantula to use a different browser, run the following command:

```bash
tarantula config -b <path to browser executable>
```
