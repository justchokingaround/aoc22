{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  packages = with pkgs; [
    # rust stuff
    cargo
    rustc
    rust-analyzer
    rustfmt
    clippy

    # openssl
    openssl
    pkg-config

    # lobster stuff
    fzf
    chafa
    rofi
    mpv
  ];
}
