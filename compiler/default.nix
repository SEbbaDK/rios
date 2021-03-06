{ pkgs ? import ./pkgs.nix { }
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

  # Disable tests while the spec causes and impure path error
  doCheck = false;
}
