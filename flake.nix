{
  description = "A development shell for the HackerNews Pipeline project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          # For Rust development
              pkgs.rustc
              pkgs.cargo
          pkgs.pkg-config
          pkgs.openssl

          # For Python development
          pkgs.python3
          pkgs.python3Packages.pip
          pkgs.python3Packages.duckdb
          pkgs.python3Packages.pandas

          # For version control
          pkgs.git
        ];

        shellHook = ''
          echo "Entering Flake-based development shell for HackerNews Pipeline..."
          echo "Available tools: rustc, cargo, python, pip, git"
        '';
      };
    };
}
