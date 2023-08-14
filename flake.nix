{
  description = "Build a cargo project without extra checks";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        craneLib = crane.lib.${system};
        gen-args = {
          buildInputs = [
            # Add additional build inputs here
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ];

          # Additional environment variables can be set directly
          # MY_CUSTOM_VAR = "some value";
        };
        cli-crate = craneLib.buildPackage (gen-args // {
          src = craneLib.cleanCargoSource (craneLib.path ./src/catppuccinifier-cli);
        });
        #gui-crate = craneLib.buildPackage (gen-args // {
        #  src = craneLib.cleanCargoSource (craneLib.path ./src/catppuccinifier-gui);
        #  buildInputs = [
        #    pkgs.nodejs
        #    pkgs.cargo-tauri
        #  ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
        #    pkgs.libiconv
        #  ];
        #});
      in
      {
        checks = {};

        packages.cli = cli-crate;
        #packages.gui = gui-crate;

        apps.cli = flake-utils.lib.mkApp {
          drv = cli-crate;
        };

        #apps.gui = flake-utils.lib.mkApp {
        #  drv = gui-crate;
        #};

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks.${system};

          # Additional dev-shell environment variables can be set directly
          # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

          # Extra inputs can be added here
          nativeBuildInputs = with pkgs; [
            cargo
            rustc
            #nodejs
            #cargo-tauri
          ];
        };
      });
}
