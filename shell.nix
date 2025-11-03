{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    xorg.libX11
    xorg.libXi
  ];
  buildInputs = with pkgs; [
  ];
}
