{
  saltz_unwrapped,
}:

let
  #inherit (lib);
in 
  saltz_unwrapped
  
  #  runCommand saltz_unwrapped.name 
  #{
  #  inherit (saltz_unwrapped) pname version meta;
  #  #nativeBuildInputs = [makeWrapper];
  #}
  #''
  #  mkdir -p $out/bin 
  #  ln -s ${saltz_unwrapped}/bin/saltz_unwrapped $out/bin/saltz_unwrapped 
  #''


  #ln -s ${saltz_unwrapped}/share $out/share
  #makeWrapper ${saltz_unwrapped}/bin/saltz $out/bin/saltz

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
