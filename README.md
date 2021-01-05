# Mod Organizer for Linux

## Important

This was my first project using Rust, so code quality isn't great. Turns out the language really isn't suitable for making GUIs.

Features are missing here and there, and there's some chance if you dig into the commit history, you'll find a more functional version before I ported it to use a GTK wrapper.

Take a look at the Issues page for more info.

## Overview

| Implemented | Feature | Notes |
|-------------|---------|-------|
|Done|Support Bethesda's Creation-Engine games
|Done|Natively target Linux and GTK
|Done|Use an overlay Virtual File System (VFS) to not clutter the ```Data``` folder
|Done|Integrate with Wine and Proton
|WIP|Support importing a Mod Organizer 2 (MO2) installation
|Not started|Integration with ```nexusmods.com``` (waiting for API to stabilize)
|Not started|Load order sorting using LOOT

## Installation

Run ```cargo build --release``` and the binary will be in ```target/release/mofl```.

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
