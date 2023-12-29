{
  outputs = { self, nixpkgs }:
    let pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in {
      formatter.x86_64-linux = pkgs.nixpkgs-fmt;
      devShells.x86_64-linux.default = pkgs.callPackage ./shell.nix { };
      packages.x86_64-linux.default = pkgs.callPackage ./. { };
    };
}
