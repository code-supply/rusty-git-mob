{
  outputs = { self, nixpkgs }:
    let pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in {
      formatter.x86_64-linux = pkgs.nixpkgs-fmt;
      devShells.x86_64-linux.default = pkgs.mkShell {
        packages = with pkgs; [
          cargo
          cargo-watch
          clippy
          nixpkgs-fmt
          rust-analyzer
          rustc
          rustfmt
        ];
      };

      packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage {
        name = "git-mob";
        src = ./.;
        cargoHash = "sha256-Lo79Jf4p3M8p+CiW8JQgzCoDBRDwX74zGoGWx8WzTc8=";
      };
    };
}
