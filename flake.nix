{
  description = "Rust development environment for ESP32-C6 with probe-rs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "riscv32imac-unknown-none-elf" ];
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "esp32c6-probe-rs";

          nativeBuildInputs = with pkgs; [
            rustToolchain
            pkg-config
            probe-rs-tools
          ];
        };
      });
}
