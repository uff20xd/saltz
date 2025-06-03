{ overlays }:

{
  saltz = import ./saltz.nix;
  overlayNixpkgsForThisInstance =
    { pkgs, ... }:
    {
      nixpkgs = {
        inherit overlays;
      };
  };


}
