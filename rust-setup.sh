echo "Checking the current Rust version..."
rustc --version

echo "Checking the current Cargo version..."
cargo --version 

# echo "Setting up the Rust environment for the project..."
# # Ensure the Rust toolchain is up to date
# rustup update

echo "Cleaning up the Cargo environment..."
rm -f Cargo.lock

echo "Installing project dependencies..."
cargo install