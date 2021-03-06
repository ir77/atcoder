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

## install rustup
curl https://sh.rustup.rs -sSf | sh -s -- -y 

## setup and install starship
## curl -fsSL https://starship.rs/install.sh | bash
