{ rustPlatform, lib, ... }:

rustPlatform.buildRustPackage {
  pname = "setup-env";
  version = "1.0";

  src = ./.;

  cargoHash = "sha256-hIlmrnfvunvvBJF7DAajSH4lKED9GouVlH+f2nOAbeM=";

  installPhase = ''
    runHook preInstall
    mkdir -p $out/envs
    mkdir -p $out/bin/
    cp -r target/x86_64-unknown-linux-gnu/release/setup-env $out/bin/
    cp -r envs/* $out/envs/
    runHook postInstall
  '';

  meta = with lib; {
    description = "A program to setup a developement environment in nixos";
    license = licenses.gpl2;
    platforms = platforms.all;
  };
}
