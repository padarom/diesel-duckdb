{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
        with pkgs; {
         # formatter = alejandra;

          devShells.default = mkShell {
            RUST_SRC_PATH = "${rust.packages.stable.rustPlatform.rustLibSrc}";

            nativeBuildInputs = [
              pkg-config
              (diesel-cli.override {
                sqliteSupport = false;
                mysqlSupport = false;
              })
            ];

            buildInputs = [
              (rust-bin.stable.latest.rust.override {
                extensions = ["rust-src"];
              })
              dbus
            ];
          };
        }
    );
}
