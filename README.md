# Mod Organizer for Linux

## Overview

| Implemented | Feature |
|-------------|---------|
|:white_check_mark:|Support Bethesda's Creation-Engine games
|:white_check_mark:|Natively target Linux and GTK
|:white_check_mark:|Use an overlay Virtual File System (VFS) to not clutter the ```Data``` folder
|:white_check_mark:|Integrate with Wine and Proton
|⬜️|Support importing a Mod Organizer 2 (MO2) installation
|⬜️|Integration with ```nexusmods.com``` (waiting for API to stabilize)
|⬜️|Load order sorting using LOOT

## Installation

(Will update once distributions start packaging mofl)

If you have Rust, run ```cargo build --release``` and the binary will be in ```target/release/mofl```.

If you don't have Rust, prebuilt binaries ~~are available~~ TODO

## Minimum Requirements

| Name | Version | Reason |
|------|---------|--------|
|GTK|>=3.18|Interface|
|7-Zip|any|Mod installation|
|Steam|Late 2018|Proton, Wine runtimes|
|libfuse|>=3|VFS|
|fuse-overlayfs|>=0.3|VFS|

## Optional Requirements

| Name | Version | Reason |
|------|---------|--------|
|Rust|>=1.31|Rust 2018, Building|

```/tmp``` should be writable by your user account.

## License

Due to this program relying on GPL-ed programs for its core functionalities, and developing alternatives will take too much time, I've decided to also license it under the GPLv3 to not cause any headaches later.