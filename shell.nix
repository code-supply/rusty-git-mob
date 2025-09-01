{
  lib,
  pkgs,
}:

with lib.lists;
with pkgs;

mkShell {
  shellHook = ''
    export OPENSSL_DEV=${openssl.dev}
  '';

  packages = [
    cargo
    cargo-watch
    clippy
    ncurses
    nixpkgs-fmt
    openssl
    pkg-config
    rust-analyzer
    rustc
    rustfmt
    zlib.dev
  ] ++ (optional stdenv.isDarwin libiconv);
}
