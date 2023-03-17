{
  description = "ZeroTier interface name Evaluator";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: let
    supportedSystems = with flake-utils.lib.system; [
      aarch64-linux
      x86_64-linux
      x86_64-freebsd
    ];
    inherit (flake-utils.lib) eachSystem;
  in eachSystem supportedSystems (system: {
    packages = let
      inherit (nixpkgs) lib;
      inherit (nixpkgs.legacyPackages.${system}.rustPlatform) buildRustPackage;
    in {
      default = buildRustPackage {
        pname = "ztifname";
        version = "0.0.1-SNAPSHOT";

        src = self;
        cargoLock.lockFile = ./Cargo.lock;

      };
    };
  });
}
