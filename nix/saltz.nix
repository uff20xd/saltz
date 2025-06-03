{
  config,
  pkgs,
  lib,
  makeWrapper,
  runCommand,
  saltz_unwrapped,
}:

let
  #inherit (lib);
in 
  runCommand saltz_unwrapped.name 
{
  inherit (saltz_unwrapped) pname version meta;
  nativeBuildInputs = [makeWrapper];
}
''
  mkdir -p $out/bin 
  echo $out
  ln -s ${saltz_unwrapped}/share $out/share
  ln -s ${saltz_unwrapped}/bin/saltz $out/bin/saltz
''

#with lib;
#let
#  pgn = config.programs.saltz;
#in 
#  { 
#  options = {
#    programs.saltz = rec {
#      enable = mkOption{
#        type = types.bool;
#        default = false;
#      };
#    };
#  };
#  config = mkIf pgn.enable {
#    environment.systemPackages = [ pkgs.saltz ];
#  };
#}
