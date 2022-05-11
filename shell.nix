let
  # An up-to-date package for enabling OpenGL support in Nix
  nixgl = import (fetchTarball "https://github.com/guibou/nixGL/archive/master.tar.gz") { };

  # Pinned version of the nix package repository that has Godot 3.4.0 and compatible with godot-rust 0.10.0
  pkgs = import (fetchTarball "https://github.com/nixos/nixpkgs/archive/nixos-21.11.tar.gz") { };
in
# Configure the dependency of your shell
  # Add support for clang for bindgen in godot-rust
pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
  buildInputs = [
    pkgs.openssl

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

# project.pkgs.stdenv.mkDerivation
# {
#   name = "slippery";
#   nativeBuildInputs = with project.pkgs; [
#     cacert
#     emacs
#     godot
#     jdk
#     nix
#     openssl
#     pkgconfig
#     sccache
#   ];
#   buildInputs = with project.pkgs; [
#     alsaLib
#     clang
#     glibc_multi
#     libGL
#     libpulseaudio
#     xorg.libX11
#     xorg.libXcursor
#     xorg.libXi
#     xorg.libXinerama
#     xorg.libXrandr
#     xorg.libXrender
#     zlib
#   ]
#   ++ builtins.attrValues project.devTools;
#   shellHook = ''
#     export PATH=$HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:$PATH
#     ${project.ci.pre-commit-check.shellHook}
#   '';

#   # Set Environment Variables
#   EDITOR = "emacs";
#   LD_LIBRARY_PATH = builtins.concatStringsSep ":" (with project.pkgs; [
#     "${alsaLib}/lib/"
#     "${libGL}/lib/"
#     "${libpulseaudio}/lib/"
#     "${xorg.libX11}/lib/"
#     "${xorg.libXcursor}/lib/"
#     "${xorg.libXi}/lib/"
#     "${xorg.libXinerama}/lib/"
#     "${xorg.libXrandr}/lib/"
#     "${xorg.libXrender}/lib/"
#     "${zlib}/lib/"
#   ]);
#   LIBCLANG_PATH = "${project.pkgs.llvmPackages.libclang}/lib";
#   RUST_BACKTRACE = 1;
#   RUSTC_WRAPPER = "${project.pkgs.sccache}/bin/sccache";
# }
