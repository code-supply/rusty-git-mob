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
          ncurses
          nixpkgs-fmt
          rust-analyzer
          rustc
          rustfmt
        ];
      };

      packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage {
        name = "git-mob";
        src = ./.;
        cargoHash = "sha256-SdHL1FBowRMgbl5S3gUJgDCPBRM9WWNFZ5hEV265XO0=";
        buildInputs = [ pkgs.ncurses ];
      };
    };
}
