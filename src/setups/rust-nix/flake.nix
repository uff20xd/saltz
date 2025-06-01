{
  description = "Foo Bar";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = forAllSystems (
        system: 
        import nixpkgs {
          inherit system;
          overlays = overlayList;
        }
      );

      overlayList = [ self.overlays.default ];
    in {

      overlays.default = final: prev: { saltz = final.callPackage ./package.nix { }; };

      packages = forAllSystems (system: {
        default = pkgsFor.${system}.project_name;
        project_name = pkgsFor.${system}.project_name;
      });

      nixosModules = import ./modules { overlays = overlayList; };

      devShells = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./shell.nix { };
      });
    };
}

