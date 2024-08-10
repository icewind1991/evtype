{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    cross-naersk.url = "github:icewind1991/cross-naersk";
    cross-naersk.inputs.nixpkgs.follows = "nixpkgs";
    cross-naersk.inputs.naersk.follows = "naersk";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    naersk,
    rust-overlay,
    cross-naersk,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [
          (import rust-overlay)
          (import ./nix/overlay.nix)
        ];
        pkgs = (import nixpkgs) {
          inherit system overlays;
        };
        inherit (pkgs) lib callPackage rust-bin mkShell;
        inherit (lib.sources) sourceByRegex;
        inherit (builtins) fromTOML readFile map;

        msrv = (fromTOML (readFile ./Cargo.toml)).package.rust-version;
        extractorMsrv = (fromTOML (readFile ./logging-extractor/Cargo.toml)).package.rust-version;
        msrvToolchain = rust-bin.stable."${msrv}".default;
        extractorMsrvToolchain = rust-bin.stable."${extractorMsrv}".default;

        naersk' = callPackage naersk {};
        msrvNaersk = callPackage naersk {
          rustc = msrvToolchain;
          cargo = msrvToolchain;
        };
        extractorMsrvNaersk = callPackage naersk {
          rustc = extractorMsrvToolchain;
          cargo = extractorMsrvToolchain;
        };
        cross-naersk' = pkgs.callPackage cross-naersk {inherit naersk;};

        buildMatrix = targets: {
          include =
            map (target: {
              inherit target;
              artifactSuffix = cross-naersk'.execSufficForTarget target;
            })
            targets;
        };

        hostTarget = pkgs.hostPlatform.config;
        targets = [
          "x86_64-unknown-linux-musl"
          hostTarget
        ];
        releaseTargets = lib.lists.remove hostTarget targets;

        src = sourceByRegex ./. ["Cargo.*" "(src)(/.*)?"];
        nearskOpt = {
          pname = "evtype";
          root = src;
        };
      in rec {
        packages =
          lib.attrsets.genAttrs targets (target:
            (cross-naersk'.buildPackage target) nearskOpt)
          // {
            inherit (pkgs) evtype;
            check = msrvNaersk.buildPackage (nearskOpt
              // {
                mode = "check";
              });
            clippy = msrvNaersk.buildPackage (nearskOpt
              // {
                mode = "clippy";
              });
            default = pkgs.evtype;
          };
        apps.default = packages.default;

        matrix = buildMatrix targets;
        releaseMatrix = buildMatrix releaseTargets;

        devShells.default = mkShell {
          nativeBuildInputs = with pkgs; [msrvToolchain rustc bacon cargo-msrv cargo-insta samply];
        };
      }
    )
    // {
      overlays.default = import ./nix/overlay.nix;
    };
}
