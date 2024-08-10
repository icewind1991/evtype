{
  stdenv,
  rustPlatform,
  lib,
}: let
  inherit (lib.sources) sourceByRegex;
in
  rustPlatform.buildRustPackage rec {
    pname = "evtype";
    version = "0.1.0";

    src = sourceByRegex ../. ["Cargo.*" "(src)(/.*)?"];

    cargoLock = {
      lockFile = ../Cargo.lock;
    };
  }
