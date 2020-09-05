## update and install some things we should probably have
apt-get update
apt-get install -y \
  curl \
  git \
  gnupg2 \
  jq \
  sudo \
  zsh \
  vim \
  build-essential \
  openssl

## Install rustup and common components
curl https://sh.rustup.rs -sSf | sh -s -- -y 
rustup component add rustfmt
rustup component add clippy 

cargo install cargo-expand
cargo install cargo-edit

## setup and install starship
## curl -fsSL https://starship.rs/install.sh | bash
