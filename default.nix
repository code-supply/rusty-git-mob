{ lib
, pkgs
}:

with lib.lists;
with pkgs;

rustPlatform.buildRustPackage {
  name = "rusty-git-mob";
  src = ./.;
  cargoHash = "sha256-cvBenRifTewzZMd+UShKFvWUQ/sDSYDXWlhfbGgqN7g=";

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ ncurses openssl ] ++
    optional stdenv.isDarwin darwin.apple_sdk.frameworks.Security;
}
