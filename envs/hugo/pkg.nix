{ pkgs, inputs, ... }:

pkgs.stdenv.mkDerivation {
  pname = "hugo-website";
  version = "1.0";

  src = ./.;

  buildInputs = with pkgs; [ hugo ];

  # configurePhase = ''
  #   mkdir -p themes/PaperMod

  #   cp -r ${inputs.papermod-theme}/* themes/PaperMod
  # '';

  buildPhase = ''
    hugo build
  '';

  installPhase = ''
    runHook preInstall
    mkdir -p $out
    cp -r public/* $out
    runHook postInstall
  '';
}
