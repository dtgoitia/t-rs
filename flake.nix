{
  description = "Toggl CLI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        package_name = "t";
        cargoTomlFile = ./Cargo.toml;
        cargoLockFile = ./Cargo.lock;

        pkgs = nixpkgs.legacyPackages.${system};

        cargoToml = builtins.fromTOML (builtins.readFile cargoTomlFile);
        cargoTomlVersion = cargoToml.package.version;

        cargoLock = builtins.fromTOML (builtins.readFile cargoLockFile);
        cargoLockVersion = (builtins.head (builtins.filter (p: p.name == package_name) cargoLock.package)).version;
      in {
        # https://msfjarvis.dev/posts/writing-your-own-nix-flake-checks/
        checks = {
          versions-must-align =
            pkgs.runCommandLocal "versions-must-align" {
              src = ./.;
            } ''
              mkdir "$out"
              if [[ "${cargoTomlVersion}" == "${cargoLockVersion}" ]]; then
                echo "versions match :P"
                exit 0;
              else
                echo 'Versions must be the same in ${cargoTomlFile} and ${cargoLockFile}, but instead got:'
                echo '  - ${cargoTomlFile}: ${cargoTomlVersion}'
                echo '  - ${cargoLockFile}: ${cargoLockVersion}'
                exit -1;
              fi
            '';
        };

        packages.default = pkgs.rustPlatform.buildRustPackage rec {
          pname = package_name;
          version = cargoTomlVersion;
          src = ./.;
          cargoToml = cargoTomlFile;
          cargoLock.lockFile = cargoLockFile;

          nativeInputs = [
            pkgs.pkg-config
            pkgs.rustPlatform.bindgenHook
          ];

          buildInputs = [
            pkgs.openssl
          ];

          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.cargo
            pkgs.openssl
            pkgs.pkg-config
            pkgs.rustc
          ];

          shellHook = ''
            exec fish
          '';
        };
      }
    );
}
