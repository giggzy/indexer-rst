with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [

    # rust 
    rustc 
    cargo
    rustfmt
    rust-analyzer
    rust-code-analysis

    # tools
    neovim
    vim
    jq
    vscode
    direnv
    git
    zoxide
    fzf
    zip
    unzip
    curl
    ripgrep
    
    # python
    python3

    # Example Build-time Additional Dependencies
    pkgconfig
  ];
  buildInputs = [
    # Example Run-time Additional Dependencies
    openssl
    libiconv  # This is OSX specific and could cause issue on other systems?

  ];

  # Set Environment Variables
  #RUST_BACKTRACE = 1;
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
