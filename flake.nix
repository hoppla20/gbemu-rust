{
  description = "Gameboy emulator written in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import rust-overlay)];
        };

        rust-pkg = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["llvm-tools-preview"];
        };
      in {
        formatter = pkgs.alejandra;

        devShells.default = pkgs.mkShell rec {
          nativeBuildInputs =
            builtins.attrValues {
              inherit
                (pkgs)
                trunk
                pkg-config
                cargo-nextest
                ;
            }
            ++ [rust-pkg]
            ++ pkgs.lib.optionals (pkgs.stdenv.isLinux) [
              pkgs.cargo-llvm-cov
            ];

          buildInputs = builtins.attrValues {
            inherit
              (pkgs)
              openssl
              libxkbcommon
              libGL
              fontconfig
              wayland
              ;

            inherit
              (pkgs.xorg)
              libXcursor
              libXrandr
              libXi
              libX11
              ;
          };

          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
        };
      }
    );
}
