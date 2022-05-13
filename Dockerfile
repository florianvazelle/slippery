FROM rust:1.60

# Variables
ARG GODOT_VERSION="3.4.2"
ENV DEBIAN_FRONTEND=noninteractive

# Updates and installs
RUN apt-get update
RUN apt-get install -y unzip
RUN apt-get install -y wget
RUN apt-get install -y curl

# Download Godot and export template, version is set from variables
RUN wget https://downloads.tuxfamily.org/godotengine/${GODOT_VERSION}/Godot_v${GODOT_VERSION}-stable_linux_headless.64.zip \
    && wget https://downloads.tuxfamily.org/godotengine/${GODOT_VERSION}/Godot_v${GODOT_VERSION}-stable_export_templates.tpz \
    && mkdir ~/.cache \
    && mkdir -p ~/.config/godot \
    && mkdir -p ~/.local/share/godot/templates/${GODOT_VERSION}.stable \
    && unzip Godot_v${GODOT_VERSION}-stable_linux_headless.64.zip \
    && mv Godot_v${GODOT_VERSION}-stable_linux_headless.64 /usr/local/bin/godot \
    && unzip Godot_v${GODOT_VERSION}-stable_export_templates.tpz \
    && mv templates/* ~/.local/share/godot/templates/${GODOT_VERSION}.stable \
    && rm -f Godot_v${GODOT_VERSION}-stable_export_templates.tpz Godot_v${GODOT_VERSION}-stable_linux_headless.64.zip

# Install exportable target
RUN rustup target add x86_64-unknown-linux-gnu

# Install cargo-make
RUN apt-get install -y libssl-dev pkg-config
RUN cargo install --force cargo-make

# Install bindgen dependencies
RUN apt-get install -y librust-bindgen-dev