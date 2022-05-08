{ project ? import ./nix { } }:

project.pkgs.stdenv.mkDerivation
{
  name = "slippery";
  nativeBuildInputs = with project.pkgs; [
    cacert
    emacs
    godot
    jdk
    nix
    openssl
    pkgconfig
    sccache
  ];
  buildInputs = with project.pkgs; [
    alsaLib
    clang
    glibc_multi
    libGL
    libpulseaudio
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXinerama
    xorg.libXrandr
    xorg.libXrender
    zlib
  ]
  ++ builtins.attrValues project.devTools;
  shellHook = ''
    export PATH=$HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:$PATH
    ${project.ci.pre-commit-check.shellHook}
  '';

  # Set Environment Variables
  EDITOR = "emacs";
  LD_LIBRARY_PATH = builtins.concatStringsSep ":" (with project.pkgs; [
    "${alsaLib}/lib/"
    "${libGL}/lib/"
    "${libpulseaudio}/lib/"
    "${xorg.libX11}/lib/"
    "${xorg.libXcursor}/lib/"
    "${xorg.libXi}/lib/"
    "${xorg.libXinerama}/lib/"
    "${xorg.libXrandr}/lib/"
    "${xorg.libXrender}/lib/"
    "${zlib}/lib/"
  ]);
  LIBCLANG_PATH = "${project.pkgs.llvmPackages.libclang}/lib";
  RUST_BACKTRACE = 1;
  RUSTC_WRAPPER = "${project.pkgs.sccache}/bin/sccache";
}
