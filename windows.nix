with import <nixpkgs> {
  crossSystem = {
    config = "x86_64-w64-mingw32";
  };
};
pkgs.mkShell {
  buildInputs = with pkgs; [
    windows.pthreads
  ];
}
