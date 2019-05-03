# Mod Organizer for Linux

## Overview

The goal of this project is to provide a mod organizer which does the following:

- Supports Bethesda's Creation-Engine games

- Natively targets Linux and GTK

- Uses an overlay Virtual File System (VFS) to not clutter the ```Data``` folder

- Integrates with Wine and Proton

- Supports importing a Mod Organizer 2 (MO2) installation

## Installation

(Will update once distributions start packaging mofl)

If you have Rust, run ```cargo build --release``` and copy ```target/release/mofl``` to ```~/.cargo/bin/```

If you don't have Rust, grab one of the precompiled binaries from the Releases page and put it in ```/usr/bin/```

## Minimum Requirements

| Name | Version | Reason |
|------|---------|--------|
|GTK|>=3.18|Interface|
|7-Zip|any|Mod installation|
|Steam|Late 2018|Launching games through Proton|
|libfuse|>=3|VFS|
|fuse-overlayfs|>=0.3|VFS|

## Optional Requirements

| Name | Version | Reason |
|------|---------|--------|
|Rust|>=1.31|Building|

```/tmp``` should be writable by your user account.

## Roadmap

- Integration with ```nexusmods.com``` (waiting for API to stabilize)

- Load order sorting using LOOT

## License

I personally prefer the MIT license over GPL, but this program relies on GPL-ed programs for its core functionalities, and developing alternatives will take too much time.

So GPL it is.