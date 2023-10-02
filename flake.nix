{
  outputs = { self, nixpkgs }: 
    let pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in {
    devShells.x86_64-linux.default = pkgs.mkShell {
      packages = [
        pkgs.cargo
        pkgs.rustc
      ];
    };

    packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage {
      name = "git-mob";
      src = ./.;
      cargoSha256 = "sha256-V6KZj6gPeUarnuy+1Wes/9H39l8vjfZUHIzGqfYV0aA=";

    };
  };
}
