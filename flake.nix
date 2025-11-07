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
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        pkgsCross = import nixpkgs {
          inherit system overlays;
          crossSystem = {
            config = "x86_64-w64-mingw32";
          };
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain (
          p:
          p.rust-bin.stable.latest.default.override {
            extensions = [
              "clippy"
              "rustfmt"
            ];
            targets = [ "x86_64-pc-windows-gnu" ];
          }
        );
        craneLibCross = (crane.mkLib pkgsCross).overrideToolchain (
          p:
          p.rust-bin.stable.latest.default.override {
            targets = [ "x86_64-pc-windows-gnu" ];
          }
        );

        smtpa = craneLib.buildPackage (
          let
            mingw = pkgs.pkgsCross.mingwW64;
          in
          {
            src = craneLib.cleanCargoSource ./.;
            nativeBuildInputs = with pkgs; [
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
        smtpaWindows = craneLibCross.buildPackage {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;
        };
      in
      {
        packages = {
          default = smtpa;
          windows = smtpaWindows;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = smtpa;
        };

        devShells.default = craneLib.devShell ({
          inputsFrom = [ smtpa ];
        });

        formatter = pkgs.nixfmt-rfc-style;
      }
    )
    // {
      nixosModules.smtpa =
        {
          lib,
          pkgs,
          config,
          ...
        }:
        let
          cfg = config.programs.supermarketTogetherPricingAccessibility;
        in
        {
          options.programs.supermarketTogetherPricingAccessibility = {
            enable = lib.mkEnableOption "supermarketTogetherPricingAccessibility";
            inputTest.enable = lib.mkEnableOption "inputTest";
            package = lib.mkOption {
              type = lib.types.package;
              description = "Derivation to install for supermarketTogetherPricingAccessibility";
              default = self.packages.${pkgs.system}.default;
            };
          };
          config = lib.mkIf cfg.enable (
            let
              smtpa = pkgs.writeShellScriptBin "supermarket-together-pricing-accessibility" ''
                #!${pkgs.runtimeShell}
                exec ${cfg.package}/bin/supermarket_together_pricing_accessibility "$@"
              '';
              inputTest = pkgs.writeShellScriptBin "supermarket-together-pricing-accessibility-input-test" ''
                #!${pkgs.runtimeShell}
                exec ${cfg.package}/bin/input_test "$@"
              '';
            in
            {
              environment.systemPackages = [
                smtpa
              ]
              ++ lib.optional cfg.inputTest.enable inputTest;
            }
          );
        };
    };
}
