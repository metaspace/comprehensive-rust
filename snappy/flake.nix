{
  description = "Flake snappy example";
  inputs.nixpkgs.url = "nixpkgs/nixos-24.11";
  inputs.systems.url = "github:nix-systems/default";
  inputs.flake-utils = {
    url = "github:numtide/flake-utils";
    inputs.systems.follows = "systems";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.snappy
            pkgs.pkg-config
          ];
          LIBCLANG_PATH = "${pkgs.llvmPackages_18.libclang.lib}/lib";
          BINDGEN_EXTRA_CLANG_ARGS = "-isystem ${pkgs.clang}/resource-root/include $NIX_CFLAGS_COMPILE";
        };
      }
    );
}
