let
    # An up-to-date package for enabling OpenGL support in Nix
    nixgl = import (fetchTarball "https://github.com/guibou/nixGL/archive/master.tar.gz") {};

    # Pinned version of the nix package repository that has Godot 3.4 and compatible with godot-rust 0.10.0
    pkgs = import (fetchTarball "https://github.com/nixos/nixpkgs/archive/nixos-21.11.tar.gz") {};
in
    # Configure the dependency of your shell
    # Add support for clang for bindgen in godot-rust
    pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
        buildInputs = [
            # Rust related dependencies
            pkgs.rustc
            pkgs.cargo
            pkgs.rustfmt
            pkgs.libclang

            # Godot Engine Editor
            pkgs.godot

            # The support for OpenGL in Nix
            nixgl.auto.nixGLDefault
        ];

        # Point bindgen to where the clang library would be
        LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

        # For Rust language server and rust-analyzer
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

        # Alias the godot engine to use nixGL
        shellHook = ''
            alias godot="nixGL godot -e"
        '';
    }