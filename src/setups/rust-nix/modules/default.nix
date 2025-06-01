{ overlays }:

{
  project_name = import ./project_name.nix;
  overlayNixpkgsForThisInstance =
    { pkgs, ... }:
    {
      nixpkgs = {
        inherit overlays;
      };
  };


}
