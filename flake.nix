{
  outputs = { self, nixpkgs }:
    let pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in {
      formatter.x86_64-linux = pkgs.nixpkgs-fmt;
      devShells.x86_64-linux.default = pkgs.mkShell {
        packages = with pkgs; [
          cargo
          nixpkgs-fmt
          rust-analyzer
          rustc
        ];
      };

      packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage {
        name = "git-mob";
        src = ./.;
        cargoHash = "sha256-5Lckrq0+IMszE39HQoCXoUTh3uzB+m4JyOCtuPc1UvY=";
      };
    };
}
