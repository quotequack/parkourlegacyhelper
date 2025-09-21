{
  description = "A Rust powered helper for the parkour legacy game on roblox";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: 
    flake-utils.lib.eachDefaultSystem (system:
      let 
        pkgs = import nixpkgs { inherit system; };
        rust = pkgs.rustPlatform;

      in {
        packages.default = rust.buildRustPackage {
          pname = "parkour-helper";
          version = "0.1.0";
          
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          nativeBuildInputs = [ pkgs.libinput.dev pkgs.pkg-config ];
          buildInputs = [ pkgs.xdotool pkgs.xorg.libXi pkgs.gcc pkgs.libinput.dev pkgs.systemdMinimal pkgs.xorg.libX11.dev pkgs.xorg.libXtst ];
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.xdotool
            pkgs.gcc
            pkgs.libinput.dev
            pkgs.systemdMinimal
            pkgs.xorg.libXtst
            pkgs.xorg.libX11.dev
            pkgs.pkg-config
            pkgs.cargo
            pkgs.rustc
            pkgs.rust-analyzer
            pkgs.xorg.libXi
          ];
        };
      }
    );
}

