{
  config,
  pkgs,
  lib ? pkgs.lib,
  ...
}:
with lib; 

let
  pgn = config.programs.saltz;
in 
  { 
  options = {
    programs.saltz = rec {
      enable = mkOption{
        type = types.bool;
        default = false;
      };
    };
  };
  config = mkIf pgn.enable {
    environment.systemPackages = [ pkgs.saltz ];
  
  };

}
