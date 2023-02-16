{
  description = "VESA Monitor Control Command Set";
  inputs = {
    flakelib.url = "github:flakelib/fl";
    nixpkgs = { };
    rust = {
      url = "github:arcnmx/nixexprs-rust";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, flakelib, nixpkgs, rust, ... }@inputs: let
    nixlib = nixpkgs.lib;
  in flakelib {
    inherit inputs;
    systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
    devShells = {
      plain = {
        mkShell
      , enableRust ? true, cargo
      , rustTools ? [ ]
      , generate
      }: mkShell {
        inherit rustTools;
        nativeBuildInputs = nixlib.optional enableRust cargo ++ [
          generate
        ];
      };
      stable = { rust'stable, outputs'devShells'plain }: outputs'devShells'plain.override {
        inherit (rust'stable) mkShell;
        enableRust = false;
      };
      dev = { rust'unstable, outputs'devShells'plain }: outputs'devShells'plain.override {
        inherit (rust'unstable) mkShell;
        enableRust = false;
        rustTools = [ "rust-analyzer" ];
      };
      default = { outputs'devShells }: outputs'devShells.plain;
    };
    checks = {
      rustfmt = { rust'builders, source }: rust'builders.check-rustfmt-unstable {
        src = source;
        config = ./.rustfmt.toml;
      };
      versions = { rust'builders, source }: rust'builders.check-contents {
        src = source;
        patterns = [
          { path = "src/lib.rs"; docs'rs = {
            inherit (self.lib.crate) name version;
          }; }
          { path = "caps/src/lib.rs"; docs'rs = {
            inherit (self.lib.crate.members.caps) name version;
          }; }
          { path = "db/src/lib.rs"; docs'rs = {
            inherit (self.lib.crate.members.db) name version;
          }; }
        ];
      };
      test = { rustPlatform, source }: rustPlatform.buildRustPackage rec {
        pname = self.lib.crate.package.name;
        inherit (self.lib.crate) cargoLock version;
        src = source;
        cargoBuildFlags = [ "--workspace" ];
        cargoTestFlags = cargoBuildFlags;
        buildType = "debug";
        meta.name = "cargo test";
      };
    };
    legacyPackages = { callPackageSet }: callPackageSet {
      source = { rust'builders }: rust'builders.wrapSource self.lib.crate.src;

      generate = { rust'builders, outputHashes }: rust'builders.generateFiles {
        paths = {
          "lock.nix" = outputHashes;
        };
      };
      outputHashes = { rust'builders }: rust'builders.cargoOutputHashes {
        inherit (self.lib) crate;
      };
    } { };
    lib = with nixlib; {
      crate = rust.lib.importCargo {
        inherit self;
        path = ./Cargo.toml;
        inherit (import ./lock.nix) outputHashes;
      };
      inherit (self.lib.crate.package) version;
    };
    config = rec {
      name = "mccs-rs";
      packages.namespace = [ name ];
    };
  };
}
