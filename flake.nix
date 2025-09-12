{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    naersk.url = "github:nix-community/naersk";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    inputs@{
      nixpkgs,
      flake-parts,
      naersk,
      rust-overlay,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      perSystem =
        { system, ... }:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          naersk' = pkgs.callPackage naersk {
            rustc = rust-toolchain;
            cargo = rust-toolchain;
          };
          commonBuildInputs = [
            pkgs.mold
          ];
          env.RUSTFLAGS = "-Clink-args=-fuse-ld=mold";
        in
        {
          packages.serverlumen = naersk'.buildPackage {
            name = "serverlumen";
            version = "0.1.0";
            src = ./serverlumen;
            root = ./.;
            nativeBuildInputs = [ ] ++ commonBuildInputs;
            inherit (env) RUSTFLAGS;
          };

          devShells.default = pkgs.mkShell {
            name = "lumen";
            nativeBuildInputs = [
              rust-toolchain
            ]
            ++ commonBuildInputs;
            inherit (env) RUSTFLAGS;
          };
        };
    };
}
