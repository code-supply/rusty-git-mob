{
  outputs = { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      forAllSystems = generate: nixpkgs.lib.genAttrs [
        "aarch64-darwin"
        "x86_64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ]
        (system: generate (
          let
            pkgs = nixpkgs.legacyPackages.${system};
            callPackage = pkgs.lib.callPackageWith
              (pkgs // {
                version =
                  if self ? rev
                  then self.rev
                  else "dirty";
              });
          in
          {
            inherit pkgs callPackage;
          }
        ));

      packages = forAllSystems ({ callPackage, ... }: {
        default = callPackage ./. { };
      });
    in
    {
      inherit packages;

      formatter = forAllSystems ({ pkgs, ... }: pkgs.nixpkgs-fmt);

      apps = forAllSystems ({ callPackage, ... }: {
        default = {
          type = "app";
          program = callPackage ./. { };
        };
      });

      devShells = forAllSystems ({ callPackage, ... }: {
        default = callPackage ./shell.nix { };
      });

      nixosModules.homeManager = import ./home-manager-module { inherit packages; };
    };
}
