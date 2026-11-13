{
  lib,
  pkgs,
}:

with lib.lists;
with pkgs;

mkShell {
  packages = [
    cargo
    cargo-watch
    clippy
    ncurses
    nixpkgs-fmt
    pkg-config
    rust-analyzer
    rustc
    rustfmt
  ] ++ (optional stdenv.isDarwin libiconv);
}
