{ pkgs }:

with pkgs;

rustPlatform.buildRustPackage {
  name = "git-mob";
  src = ./.;
  cargoHash = "sha256-ndogLQf1Q+UPgTgRwGKxvvhnDAPBH5Ct6OLyL90STgY=";

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ ncurses openssl ];
}
