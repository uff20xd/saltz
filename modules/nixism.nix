{
  config,
  pkgs,
  lib ? pkgs.lib,
  ...
}:
with lib; 

let
  pgn = config.programs.nixism;
in 
  { 
  options = {
    programs.nixism = rec {
      enable = mkOption{
        type = types.bool;
        default = false;
      };
    };
  };
  config = mkIf pgn.enable {
    environment.systemPackages = [ pkgs.nixism ];
  
  };

}
