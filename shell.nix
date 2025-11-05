{ pkgs ? import <nixpkgs> {} }:
let
  mingwPkgs = pkgs.pkgsCross.mingwW64;
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    xorg.libX11
    xorg.libXi
    mingwPkgs.stdenv.cc
  ];

  buildInputs = [
    mingwPkgs.windows.mingw_w64_pthreads
  ];
}