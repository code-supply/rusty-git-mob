{
  lib,
  pkgs,
}:

with lib.lists;
with pkgs;

rustPlatform.buildRustPackage {
  name = "rusty-git-mob";
  src = ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [
    ncurses
    openssl
    zlib.dev
  ];
}
