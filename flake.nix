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
        cargoSha256 = "sha256-V6KZj6gPeUarnuy+1Wes/9H39l8vjfZUHIzGqfYV0aA=";
      };
    };
}
