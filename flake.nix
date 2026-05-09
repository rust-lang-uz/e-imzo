{
  description = "A beginning of an awesome project bootstrapped with github:bleur-org/templates";

  inputs = {
    nixpkgs.url = "git+https://git.oss.uzinfocom.uz/xinux/nixpkgs?ref=nixos-unstable&shallow=1";

    # The flake-utils library
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      ...
    }:
    # @ inputs
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        # Nix script formatter
        formatter = pkgs.nixfmt-tree;

        # Development environment
        devShells.default = import ./shell.nix { inherit pkgs; };
      }
    );
}
