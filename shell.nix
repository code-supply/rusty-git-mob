{ pkgs }:
pkgs.mkShell {
  shellHook = ''
    export OPENSSL_DEV=${pkgs.openssl.dev}
  '';

  packages = with pkgs; [
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
  ];
}
