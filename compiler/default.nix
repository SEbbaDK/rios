{ pkgs ? import <nixpkgs> { }
, crystal ? pkgs.crystal
, runCommand ? pkgs.runCommand
}:
let
  shard = builtins.readFile ./shard.yml;
  version = builtins.head (builtins.match ".*version: ([0-9.]+).*" shard);
in
crystal.buildCrystalPackage {
  pname = "riosc";
  inherit version;

  src = runCommand "source" { } ''
	mkdir $out
	cp -r ${./src} $out/src
	cp ${./shard.yml} $out/shard.yml
	cp ${./shard.lock} $out/shard.lock
  '';

  format = "shards";
  lockFile = ./shard.lock;
  shardsFile = ./shards.nix;

  # Disable tests until they work
  doCheck = false;
  doInstallCheck = false;

  buildPhase = "crystal build src/riosc.cr && ls -la";
  installPhase = "mkdir -p $out/bin && cp riosc $out/bin/";
}
