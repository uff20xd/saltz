{ overlays }:

{
  nixism = import ./saltz.nix;
  overlayNixpkgsForThisInstance =
    { pkgs, ... }:
    {
      nixpkgs = {
        inherit overlays;
      };
  };


}
