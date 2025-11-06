{
  description = "Accessibility tool for pricing in the game Supermarket Together";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell (
        let
          mingw = pkgs.pkgsCross.mingwW64;
        in
        {
          nativeBuildInputs = with pkgs; [
            pkg-config
            mingw.stdenv.cc
          ];

          buildInputs = with pkgs; [
            xorg.libX11
            xorg.libXi
            mingw.windows.mingw_w64_pthreads
          ];
        });
      }
    );
}