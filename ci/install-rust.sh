source ./ci/env.sh

set -e
export CARGO_HOME='/usr/local/cargo'

RUSTUP_VERSION=1.24.3
RUST_VERSION=$1
RUST_IMAGE_FULL=$2
RUST_ARCH="x86_64-unknown-linux-gnu"

RUSTUP_URL=https://static.rust-lang.org/rustup/archive/$RUSTUP_VERSION/$RUST_ARCH/rustup-init
wget $RUSTUP_URL

chmod +x rustup-init;
./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION;
rm rustup-init;
chmod -R a+w $RUSTUP_HOME $CARGO_HOME

rustup --version
cargo --version
rustc --version

if [ "$RUST_IMAGE_FULL" = "1" ]; then
  rustup component add clippy-preview
  rustup component add rustfmt
  cargo install --force cargo-deny
  cargo install --force cargo-outdated
fi

if [ "$RUST_VERSION" = "nightly" ]; then
  # Coverage tools
  cargo install grcov
  rustup component add llvm-tools-preview

  # Documentation tools
  cargo install --force rustdoc-stripper
fi
