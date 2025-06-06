{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    devenv = {
      url = "github:cachix/devenv";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    inputs@{
      nixpkgs,
      flake-parts,
      devenv,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ devenv.flakeModule ];
      systems = nixpkgs.lib.systems.flakeExposed;
      perSystem =
        { pkgs, ... }:
        {
          devenv.shells.default = {
            languages.rust = {
              enable = true;
              channel = "nightly";
              components = [
                "rustc"
                "cargo"
                "clippy"
                "rustfmt"
                "rust-src"
                "rust-analyzer"
              ];
              mold.enable = true;
            };
            languages.javascript = {
              enable = true;
              package = pkgs.nodejs-slim_23;
              pnpm.enable = true;
            };
            packages = [
              pkgs.clorinde
              pkgs.diesel-cli
              pkgs.protobuf
            ];
            git-hooks.hooks.rustfmt = {
              enable = true;
              always_run = true;
            };
          };
        };
    };
}
