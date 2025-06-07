{
  description = " A Project Manager of the worst sort";
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

      overlays.default = final: prev: { saltz = final.callPackage ./nix/saltz.nix { }; saltz_unwrapped = final.callPackage ./nix/saltz_unwrapped.nix { }; };

      packages = forAllSystems (system: {
        default = pkgsFor.${system}.saltz;
        saltz = pkgsFor.${system}.saltz;
        saltz_unwrapped = pkgsFor.${system}.saltz_unwrapped;
      });

      nixosModules = import ./modules { overlays = overlayList; };

      devShells = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./shell.nix { };
      });
    };
}

