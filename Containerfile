FROM archlinux:latest

ENV RUSTC_WRAPPER=sccache
ENV CARGO_INCREMENTAL=0
ENV CARGO_TARGET_DIR=/paxy/podman-target
ENV PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/root/.cargo/bin

# Install Rust
RUN pacman -Sy --noconfirm rustup cargo
# Toolchain setup
RUN /usr/bin/rustup self upgrade-data
RUN rustup default nightly-2024-03-17
# Project dependencies
RUN pacman -Sy --noconfirm gdk-pixbuf2 pango gtk4 pkg-config
# Extras
RUN pacman -S --noconfirm sccache
RUN cargo install cargo-make
RUN cargo install cargo-binstall
RUN cargo binstall --no-confirm wasmer-cli