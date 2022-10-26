{ config, channels, pkgs, lib, ... }: with pkgs; with lib; let
  importShell = channels.cipkgs.writeText "shell.nix" ''
    import ${builtins.unsafeDiscardStringContext config.shell.drvPath}
  '';
  build = channels.cipkgs.ci.command {
    name = "cargo-build";
    command = ''
      nix-shell ${importShell} --run "cargo build --all"
    '';
    impure = true;
  };
  test = channels.cipkgs.ci.command {
    name = "cargo-test";
    command = ''
      nix-shell ${importShell} --run "cargo test --all"
    '';
    impure = true;
  };
in {
  config = {
    name = "mccs";
    ci.gh-actions.enable = true;
    cache.cachix.arc.enable = true;
    channels = {
      nixpkgs = "22.05";
      rust = "master";
    };
    environment = {
      test = {
        inherit (config.rustChannel.buildChannel) cargo;
      };
    };
    tasks = {
      build.inputs = [ build test ];
    };
  };

  options = {
    rustChannel = mkOption {
      type = types.unspecified;
      default = channels.rust.stable;
    };
    shell = mkOption {
      type = types.unspecified;
      default = with pkgs; config.rustChannel.mkShell {
      };
    };
  };
}
