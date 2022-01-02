{
  description = "Rosalind bioinformatics";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs
            {
              overlays = [ (import rust-overlay) ];
              system = system;
            };
          lib = pkgs.lib;
          rust = pkgs.rust-bin.nightly.latest.rust;
        in
        rec
        {
          devShell = pkgs.mkShell {
            packages = with pkgs; [
              (rust.override {
                extensions = [ "rust-src" "rustfmt" ];
              })
              mold
              cargo-watch
              cargo-flamegraph
              cargo-edit
              pkgconfig
              openssl

              rnix-lsp
              nixpkgs-fmt
            ];
          };
        }
      );
}
