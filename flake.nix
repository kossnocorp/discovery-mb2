{
  description = "Rust Embedded Discovery dev environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:

    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in
      {
        devShells.default =
          with pkgs;
          let
            rust = rust-bin.stable.latest.default.override {
              extensions = [
                "rust-src"
                "llvm-tools"
              ];
              targets = [ "thumbv7em-none-eabihf" ];
            };
          in
          mkShell {
            buildInputs = [
              # System
              cacert
              openssl
              # Tools
              direnv
              usbutils
              minicom
              # Rust
              rust
              cargo-binutils
              probe-rs-tools
              # ARM
              gcc-arm-embedded
              gdb
            ];
          };
      }
    );
}
