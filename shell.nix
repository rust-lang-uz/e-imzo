{
  pkgs ?
    let
      lock = (builtins.fromJSON (builtins.readFile ./flake.lock)).nodes.nixpkgs.locked;
      nixpkgs = fetchTarball {
        url = "https://github.com/nixos/nixpkgs/archive/${lock.rev}.tar.gz";
        sha256 = lock.narHash;
      };
    in
    import nixpkgs { overlays = [ ]; },
  ...
}:
let
  # Manifest via Cargo.toml
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.mkShell {
  name = "${manifest.name}";

  # Compile time dependencies
  packages = with pkgs; [
    # Hail the Nix
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

    # Other compile time dependencies
    openssl
    pkg-config
    # libressl
  ];

  # Set Environment Variables
  RUST_BACKTRACE = "full";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
