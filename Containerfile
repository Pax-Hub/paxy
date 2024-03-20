FROM archlinux:latest

# Install Rust
RUN pacman -Sy --noconfirm rustup cargo
# Toolchain setup
RUN rustup self upgrade-data
RUN rustup default nightly-2024-03-17
# Project dependencies
RUN pacman -S --noconfirm gdk-pixbuf2 pango gtk4 pkg-config