{
  description = "Accessibility tool for pricing in the game Supermarket Together";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
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
      in
      {
        devShells.default = pkgs.mkShell (
          let
            mingw = pkgs.pkgsCross.mingwW64;
          in
          {
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
          }
        );

        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
