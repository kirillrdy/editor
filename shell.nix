{}:
let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {

  buildInputs = with pkgs; [
    rustup
    rust-analyzer
    ninja
    pkg-config
    python3
    gtk3
    meson
    libadwaita
    gtk4
  ];
}

