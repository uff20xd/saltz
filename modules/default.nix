{ overlays }:

{
  nixism = import ./nixism.nix;
  overlayNixpkgsForThisInstance =
    { pkgs, ... }:
    {
      nixpkgs = {
        inherit overlays;
      };
  };


}
