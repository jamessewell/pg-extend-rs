{ pkgsPath ? <nixpkgs>, crossSystem ? null, channel ? { channel = "stable"; } }:

let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz) ;
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };


in 
  with pkgs;
  stdenv.mkDerivation {
  name = "rust";
  nativeBuildInputs = [
    buildPackages.postgresql_11
    buildPackages.rust-bindgen
    buildPackages.cargo-edit
    buildPackages.openssl
    buildPackages.clang
    buildPackages.llvmPackages.libclang
    buildPackages.llvmPackages.libcxxStdenv
    ((buildPackages.rustChannelOf channel ).rust.override { 
      extensions = [
          "rustfmt-preview"
          "clippy-preview"
	  "rls-preview"
        ];
      })
  ];

  OPENSSL_LIB_DIR = "${buildPackages.openssl.out}/lib";
  OPENSSL_DIR = buildPackages.openssl.dev;
  LIBCLANG_PATH="${buildPackages.llvmPackages.libclang}/lib";
  PG_INCLUDE_PATH="${buildPackages.postgresql_11.out}/include/server";
}
