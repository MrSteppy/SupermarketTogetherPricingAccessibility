{
  description = "Accessibility tool for pricing in the game Supermarket Together";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      crane,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "clippy"
            "rustfmt"
          ];
          targets = [ "x86_64-pc-windows-gnu" ];
        };
        craneLib = crane.mkLib pkgs;
        mingw = pkgs.pkgsCross.mingwW64;
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          mingw.stdenv.cc
        ];
        buildInputs = with pkgs; [
          xorg.libX11
          xorg.libXi
          mingw.windows.mingw_w64_pthreads
        ];
      in
      {
        packages = {
          default = craneLib.buildPackage {
            src = craneLib.cleanCargoSource ./.;
            inherit nativeBuildInputs buildInputs;
          };
          supermarket-together-pricing-accessibility = self.packages.${system}.default;
        };

        overlays.default = final: prev: {
          inherit (self.packages.${prev.system}) supermarket-together-pricing-accessibility;
        };

        devShells.default = pkgs.mkShell ({
          inherit nativeBuildInputs buildInputs;
        });

        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
