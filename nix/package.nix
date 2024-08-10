{
  stdenv,
  rustPlatform,
  lib,
}: let
  inherit (lib.sources) sourceByRegex;
  inherit (builtins) fromTOML readFile;
in
  rustPlatform.buildRustPackage rec {
    pname = "evtype";
    version = (fromTOML (readFile ../Cargo.toml)).package.version;

    src = sourceByRegex ../. ["Cargo.*" "(src)(/.*)?"];

    cargoLock = {
      lockFile = ../Cargo.lock;
    };
  }
