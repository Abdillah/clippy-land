{
  description = "A basic flake with a shell";
  # Follow system:
  # inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.systems.url = "github:nix-systems/default";
  inputs.flake-utils = {
    url = "github:numtide/flake-utils";
    inputs.systems.follows = "systems";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            pkg-config
            libxkbcommon
            glib
            wayland
            bashInteractive 
          ];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [ wayland ]);
        };
      }
    );
}
