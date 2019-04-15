# Mod Organizer for Linux

## Overview

The goal of this project is to provide a mod organizer which does the following:

- Supports Bethesda's Creation-Engine games

- Natively targets Linux and GTK

- Uses an overlay Virtual File System (VFS) to not clutter the ```Data``` folder

- Integrates with Wine and Proton

- Supports importing a Mod Organizer 2 (MO2) installation

## Minimum Requirements

- GTK 3.18

- 7-Zip for automated mod installation

- libfuse 3 for VFS capability

- fuse-overlayfs 0.3 for VFS capability

- Rust 1.31 for building

## Roadmap

- Integration with ```nexusmods.com``` (waiting for API to stabilize)