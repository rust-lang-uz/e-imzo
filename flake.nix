{
  description = "A beginning of an awesome project bootstrapped with github:bleur-org/templates";

  inputs = {
    nixpkgs.url = "git+https://git.oss.uzinfocom.uz/xinux/nixpkgs?ref=nixos-unstable&shallow=1";

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
        formatter = pkgs.nixfmt-tree;

        devShells.default =
          let
            # Manifest via Cargo.toml
            manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
          in
          pkgs.mkShell {
            name = "${manifest.name}";

            packages = with pkgs; [
              nixd
              statix
              deadnix
              nixfmt

              # Rust
              rustc
              cargo
              rustfmt
              clippy
              rust-analyzer
              cargo-watch
              rustPlatform.bindgenHook

              openssl
              pkg-config
              # libressl
            ];

            # Set Environment Variables
            RUST_BACKTRACE = "full";
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          };
      }
    );
}
