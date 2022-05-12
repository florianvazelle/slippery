{ nixgl ? import (fetchTarball "https://github.com/guibou/nixGL/archive/master.tar.gz") { }
, lib
, stdenv 
, rustPlatform
, llvmPackages
, godot-export-templates
, godot-headless
}:

let
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));

  slippery = rustPlatform.buildRustPackage {
    pname = cargoToml.package.name;
    version = cargoToml.package.version;

    src = ./.;
    buildType = "release";
    cargoLock = {
      lockFile = ./Cargo.lock;
    };
    
    preBuild = ''
      # From: https://github.com/NixOS/nixpkgs/blob/1fab95f5190d087e66a3502481e34e15d62090aa/pkgs/applications/networking/browsers/firefox/common.nix#L247-L253
      # Set C flags for Rust's bindgen program. Unlike ordinary C
      # compilation, bindgen does not invoke $CC directly. Instead it
      # uses LLVM's libclang. To make sure all necessary flags are
      # included we need to look in a few places.
      export BINDGEN_EXTRA_CLANG_ARGS="$(< ${stdenv.cc}/nix-support/libc-crt1-cflags) \
        $(< ${stdenv.cc}/nix-support/libc-cflags) \
        $(< ${stdenv.cc}/nix-support/cc-cflags) \
        $(< ${stdenv.cc}/nix-support/libcxx-cxxflags) \
        ${lib.optionalString stdenv.cc.isClang "-idirafter ${stdenv.cc.cc}/lib/clang/${lib.getVersion stdenv.cc.cc}/include"} \
        ${lib.optionalString stdenv.cc.isGNU "-isystem ${stdenv.cc.cc}/include/c++/${lib.getVersion stdenv.cc.cc} -isystem ${stdenv.cc.cc}/include/c++/${lib.getVersion stdenv.cc.cc}/${stdenv.hostPlatform.config} -idirafter ${stdenv.cc.cc}/lib/gcc/${stdenv.hostPlatform.config}/${lib.getVersion stdenv.cc.cc}/include"} \
      "
    '';

    # Point bindgen to where the clang library would be
    LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
  };
in
  stdenv.mkDerivation {
    pname = cargoToml.package.name;
    version = cargoToml.package.version;

    src = ./.;

    nativeBuildInputs = [
      slippery

      # Godot Engine Editor
      godot-headless

      # The support for OpenGL in Nix
      nixgl.auto.nixGLDefault
    ];

    buildPhase = ''
      runHook preBuild
      
      # Cannot create file '/homeless-shelter/.config/godot/projects/...'
      export HOME=$TMPDIR
      
      # Link the export-templates to the expected location. The --export commands
      # expects the template-file at .../templates/3.2.3.stable/linux_x11_64_release
      # with 3.2.3 being the version of godot.
      mkdir -p $HOME/.local/share/godot
      ln -s ${godot-export-templates}/share/godot/templates $HOME/.local/share/godot
      
      mkdir -p lib/x86_64-unknown-linux-gnu
      cp ${slippery}/lib/*.so lib/x86_64-unknown-linux-gnu
      
      mkdir -p $out/share/slippery
      nixGL godot-headless --export "Linux/X11.x86_64-unknown-linux-gnu.release" $out/share/slippery/slippery

      runHook postBuild
    '';

    installPhase = ''
      runHook preInstall
      
      mkdir -p $out/bin
      ln -s $out/share/slippery/slippery $out/bin
      
      mkdir -p $out/share/pixmaps
      cp assets/godot-ferris-32x32.png $out/share/pixmaps/slippery.png

      runHook postInstall
    '';

    meta = with lib; {
      description = cargoToml.package.description;
      homepage = cargoToml.package.homepage;
      license = with licenses; [ unlicense ];
      platforms = [ "x86_64-linux" ];
    };
  }