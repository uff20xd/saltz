{ pkgs ? import <nixpkgs> { }}: 
pkgs.mkShell {
  #inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];
  buildInputs = with pkgs; [
    cargo
  ];
  shellHook = ''
    zsh
  '';
}
