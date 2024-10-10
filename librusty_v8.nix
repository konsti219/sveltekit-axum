# From https://github.com/NixOS/nixpkgs/blob/nixos-24.05/pkgs/development/web/deno/librusty_v8.nix
# Will need to be updated when upgrading deno deps
#
# auto-generated file -- DO NOT EDIT! (I did :3)
{
  stdenv,
  fetchurl,
}: let
  fetch_librusty_v8 = args:
    fetchurl {
      name = "librusty_v8-${args.version}";
      url = "https://github.com/denoland/rusty_v8/releases/download/v${args.version}/librusty_v8_release_${stdenv.hostPlatform.rust.rustcTarget}.a.gz";
      sha256 = args.shas.${stdenv.hostPlatform.system};
      meta = {inherit (args) version;};
    };
in
  fetch_librusty_v8 {
    version = "0.106.0";
    shas = {
      x86_64-linux = "sha256-jLYl/CJp2Z+Ut6qZlh6u+CtR8KN+ToNTB+72QnVbIKM=";
      # aarch64-linux = "sha256-rlyY4C4FMHTyPUzqHKYzToIs9tJunTXEor9wc/7zH/0=";
      # x86_64-darwin = "sha256-IUDe0ogBSCaz1q+uXepOi883hamtJYqBPtNfrm/y6Qo=";
      # aarch64-darwin = "sha256-53PuHq7AUi21cjopoFakzLuJyqSJ9VeF7g53IWxFXAI=";
    };
  }
