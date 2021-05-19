{ pkgs ? import <nixpkgs> { }
, mkDerivation ? pkgs.stdenv.mkDerivation
, cargo ? pkgs.cargo
, rustc ? pkgs.rustc
, ...
}:
let
	cargotoml = builtins.readFile ./Cargo.toml;
	version = builtins.head (builtins.match ".*version = \"([0-9.]+)\".*" cargotoml);
in
mkDerivation {
	pname = "riosc";
	inherit version;

	src = ./.;

	buildInputs = [ cargo rustc ];
}
