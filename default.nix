{
  pkgs ? import (fetchTarball "https://github.com/nixos/nixpkgs/archive/nixos-21.11.tar.gz") { },
}:

let
  inherit (pkgs) callPackage;
in
rec {
  slippery = callPackage ./slippery.nix { };
}
