{
  outputs = { self, nixpkgs }:
    let pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in {
      formatter.x86_64-linux = pkgs.nixpkgs-fmt;
      devShells.x86_64-linux.default = pkgs.mkShell {
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
      };

      packages.x86_64-linux.default = with pkgs; rustPlatform.buildRustPackage {
        name = "git-mob";
        src = ./.;
        cargoHash = "sha256-O7a9h9pGP4lEIe7Mq5ZgcKLk4rvP/JfBrqxa9GwfsHU=";

        nativeBuildInputs = [ pkg-config ];
        buildInputs = [ ncurses openssl ];
      };
    };
}
