# Extract version from Nargo.toml
VERSION=$(grep '^version = ' Nargo.toml | cut -d '"' -f 2)
echo "Circuit version: $VERSION"

# Extract version from Nargo.toml
rm -rf target

# Install Noir/Nargo
echo "Install the Noir/Nargo v1.0.0-beta.6..."
noirup --version 1.0.0-beta.6

# Align the Noir/Nargo version (1.0.0-beta.6) and bb.js version (>= 0.85.0) of the local machine.
echo "Install the bb.js version v0.85.0..."
bbup --version 0.85.0

echo "Check the Noir/Nargo version and bb.js version of the local machine (This version is supposed to be v1.0.0-beta.6 / v0.85.0)..."
nargo -V
bb -V

echo "Compiling circuit..."
if ! nargo compile; then
    echo "Compilation failed. Exiting..."
    exit 1
fi

echo "Gate count:"
bb gates -b target/verified_anonymous_sns_jwt.json | jq  '.functions[0].circuit_size'

#echo "Create version-specific directory"

echo "Create the target/vk directory..."
mkdir -p "target/vk"

echo "Copying verified_anonymous_sns_jwt.json and paste to the ./circuit directory..."
cp target/verified_anonymous_sns_jwt.json "../verified_anonymous_sns_jwt.json"

echo "Generating vkey in the ./target/vk directory..."
bb write_vk -b ./target/verified_anonymous_sns_jwt.json -o ./target/vk --oracle_hash keccak

#echo "Generating vkey.json to app/assets/jwt..."
#node -e "const fs = require('fs'); fs.writeFileSync('../app/assets/jwt/circuit-vkey.json', JSON.stringify(Array.from(Uint8Array.from(fs.readFileSync('./target/vk')))));"

echo "Generate a Solidity Verifier contract from the vkey..."
bb write_solidity_verifier -k ./target/vk/vk -o ./target/Verifier.sol

echo "Copy a Solidity Verifier contract-generated (Verifier.sol) into the ./contracts/src/circuits/zk-jwt/honk-verifier directory"
cp ./target/Verifier.sol ../../contracts/src/circuits/zk-jwt/honk-verifier

echo "Rename the Verifier.sol with the honk_vk.sol in the ./contracts/circuit/ultra-verifier directory"
mv ../../contracts/src/circuits/zk-jwt/honk-verifier/Verifier.sol ../../contracts/src/circuits/zk-jwt/honk-verifier/honk_vk.sol

echo "Done"
