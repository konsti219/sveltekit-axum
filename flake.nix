{
  description = "sveltekit-axum dev stuff";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    naersk,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      # ===================
      # General flake utils
      # ===================
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
        # config.allowUnfree = true;
      };
      inherit (pkgs) lib;

      rust_toolchain = (pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
        extensions = ["rust-std" "rust-src" "rust-analyzer"];
        targets = ["x86_64-unknown-linux-musl"];
      };

      # ===============
      # JS/Npm Packages
      # ===============

      demoapp_frontend = pkgs.buildNpmPackage {
        pname = "demoapp_frontend";
        version = "0.1.0";

        src = lib.fileset.toSource {
          root = ./.;
          fileset = ./demoapp;
        };

        npmDepsHash = "sha256-0IrCLLeRM/hI9mtwmWq9oa6e75y34toLNCEYjAIHZIQ=";
        # npmDepsHash = lib.fakeHash;

        npmPackFlags = ["--ignore-scripts"];

        postPatch = ''
          cd demoapp
        '';

        installPhase = ''
          # remove timestamp
          rm build/client/_app/version.json

          cp -r build $out
        '';
      };

      naersk' = pkgs.callPackage naersk {
        cargo = rust_toolchain;
        rustc = rust_toolchain;
      };

      demoapp_bin = naersk'.buildPackage {
        # TODO: slim fileset
        root = ./.;
        name = "demoapp";

        nativeBuildInputs = [
          # required by libz-ng-sys crate
          pkgs.cmake
        ];

        RUSTY_V8_ARCHIVE = pkgs.callPackage ./librusty_v8.nix {};
        SVELTEKIT_BUILD = demoapp_frontend;
      };
    in {
      devShells.default = pkgs.mkShell {
        name = "Dev";

        buildInputs = with pkgs; [
          rust_toolchain
          cmake

          # web toolchain
          deno
          nodejs
        ];

        shellHook = ''
          # if running from zsh, reenter zsh
          if [[ $(ps -e | grep $PPID) == *"zsh" ]]; then
            export SHELL=zsh
            zsh
            exit
          fi
        '';
      };

      packages = {
        # Binary outputs.
        inherit demoapp_bin;
        # Other outputs.
        inherit demoapp_frontend;
      };

      apps = {
        demoapp-bin = flake-utils.lib.mkApp {
          drv = demoapp_bin;
          exePath = "/bin/demoapp";
        };
      };

      formatter = nixpkgs.legacyPackages.${system}.alejandra;
    });
}
