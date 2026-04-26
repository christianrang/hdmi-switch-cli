# flake.nix
{
  description = "CLI tool for 4KMX44-H2 HDMI switch";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems =
        [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);
    in {
      packages = forAllSystems (system:
        let pkgs = nixpkgs.legacyPackages.${system};
        in {
          hdmi-switch-cli = pkgs.rustPlatform.buildRustPackage {
            pname = "hdmi-switch-cli";
            version = "1.0.1";
            src = self;
            sourceRoot = "source/hdmi-switch";
            cargoHash = "sha256-EXG088F47kU2g0dUByYzgopmjdP0b++jDlxauAR7nco=";

            meta = {
              description = "CLI tool for 4KMX44-H2 HDMI switch";
              license = nixpkgs.lib.licenses.mit;
              mainProgram = "hdmi-switch";
            };
          };

          default = self.packages.${system}.hdmi-switch-cli;
        });
    };
}
