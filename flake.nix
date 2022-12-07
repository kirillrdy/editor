{
  description = "A very basic flake";
  outputs = { self, nixpkgs }: {
    devShells.x86_64-linux.default = with import nixpkgs { system = "x86_64-linux"; }; mkShell {
      buildInputs = [
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
    };
  };
}
