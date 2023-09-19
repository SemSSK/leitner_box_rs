{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk = {
      url = "github:nix-community/naersk"; 
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils, rust-overlay, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs { inherit system overlays; };
        libPath = pkgs.lib.makeLibraryPath (with pkgs; [
          libGL
          libxkbcommon
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ]);
        naersk' = pkgs.callPackage naersk {};
      in
      {
        defaultPackage = naersk'.buildPackage {
          src = ./.;
          buildInputs = with pkgs;[
            pkg-config
            udev
            libGL
            libxkbcommon
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
          ];
        };
        packages.default =naersk'.buildPackage {
          src = ./.;
          LD_LIBRARY_PATH = libPath;
        };
        devShell = with pkgs; mkShell {
          buildInputs = [
            pkg-config
            rust-bin.stable.latest.default
            bacon
            udev
            mold
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          LD_LIBRARY_PATH = libPath;
        };
      });
}
