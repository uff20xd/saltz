{ overlays }:

{
  saltz = import ./saltz.nix;
  saltz_unwrapped = import ./saltz_unwrapped.nix;
  overlayNixpkgsForThisInstance =
    { pkgs, ... }:
    {
      nixpkgs = {
        inherit overlays;
      };
  };
}
