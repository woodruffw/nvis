nvis
====

[![Build Status](https://img.shields.io/github/workflow/status/woodruffw/nvis/CI/master)](https://github.com/woodruffw/nvis/actions?query=workflow%3ACI)

A very simple input visualizer.

[![asciicast](https://asciinema.org/a/259895.svg)](https://asciinema.org/a/259895)

## Overview

I wrote this to teach myself Rust. You probably shouldn't use it.

Ignoring that:

```bash
cargo install nvis
# or, build locally with `cargo build`
```

## Usage:

Enter input, and use these keybindings:

* `Up`/`Down`: Move between transforms
* `Ctrl-S`: Copy the currently focused transform to the system clipboard
* `Ctrl-t`: Toggle between "raw" and "smart" modes
* `Ctrl-q`: Quit
