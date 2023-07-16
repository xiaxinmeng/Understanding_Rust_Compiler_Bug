 nix
{ pkgs ? import <nixpkgs> {} }:
let
  drv = import ./. { inherit pkgs; };
  mozOverlay = import (pkgs.fetchFromGitHub {
    owner = "mozilla";
    repo = "nixpkgs-mozilla";
    rev = "e912ed483e980dfb4666ae0ed17845c4220e5e7c";
    sha256 = "08fvzb8w80bkkabc1iyhzd15f4sm7ra10jn32kfch5klgl0gj3j3";
  });
  myPkgs = import <nixpkgs> { overlays = [mozOverlay]; };
in pkgs.mkShell {
  name = "env";
  inputsFrom = [ drv ];
  buildInputs = with pkgs; [
    cargo-edit
    myPkgs.latest.rustChannels.nightly.rust
    myPkgs.latest.rustChannels.nightly.cargo
    bashInteractive
  ];
}
