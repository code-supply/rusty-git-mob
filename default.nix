{ lib
, pkgs
}:

with lib.lists;
with pkgs;

rustPlatform.buildRustPackage {
  name = "rusty-git-mob";
  src = ./.;
  cargoHash = "sha256-1GyG24NWIqiSMRk8I4t0aYXfvBX3nf5U46G1kjqiyQ0=";

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ ncurses openssl ] ++
    optional stdenv.isDarwin darwin.apple_sdk.frameworks.Security;
}
