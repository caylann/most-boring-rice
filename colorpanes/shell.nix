{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.cargo

    # keep this line if you use bash
    pkgs.bashInteractive
  ];
}
