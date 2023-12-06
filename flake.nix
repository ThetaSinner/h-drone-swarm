{
  description = "Holochain app development";

  inputs = {
    nixpkgs.follows = "holochain/nixpkgs";

    versions.url = "github:holochain/holochain?dir=versions/weekly";

    versions.inputs.holochain.url = "github:holochain/holochain/holochain-0.3.0-beta-dev.26";
    versions.inputs.lair.url = "github:holochain/lair/lair_keystore-v0.3.0";

    holochain = {
      url = "github:holochain/holochain";
      inputs.versions.follows = "versions";
    };
  };

  outputs = inputs @ { ... }:
    inputs.holochain.inputs.flake-parts.lib.mkFlake
      {
        inherit inputs;
      }
      {
        systems = builtins.attrNames inputs.holochain.devShells;
        perSystem =
          { config
          , pkgs
          , system
          , ...
          }: {
            devShells.default = pkgs.mkShell {
              inputsFrom = [ inputs.holochain.devShells.${system}.holonix ];
              packages = [ pkgs.nodejs-18_x pkgs.zip ];
              shellHook = ''
                export UI_PORT=8888
              '';
            };
          };
      };
}
