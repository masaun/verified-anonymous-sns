# Extract version from Nargo.toml
rm -rf target

# Align the Noir/Nargo version and bb.js version of the local machine.
echo "Update the bb.js version of the local machine to v0.85.0..."
bbup --version 0.85.0

echo "Check the Noir/Nargo version of the local machine (This version is supposed to be v1.0.0-beta.6)..."
nargo -V

echo "Compiling circuit..."
if ! nargo compile; then
    echo "Compilation failed. Exiting..."
    exit 1
fi

echo "Gate count:"
bb gates -b target/verified_anonymous_sns_jwt.json | jq  '.functions[0].circuit_size'

# Create version-specific directory
#mkdir -p "../app/assets/jwt"

echo "Copying verified_anonymous_sns_jwt.json to ./circuit..."
#echo "Copying circuit.json to app/assets/jwt..."
cp target/verified_anonymous_sns_jwt.json "../verified_anonymous_sns_jwt.json"
#cp target/verified_anonymous_sns_jwt.json "../app/assets/jwt/circuit.json"

echo "Generating vkey..."
bb write_vk -b ./target/verified_anonymous_sns_jwt.json -o ./target

#echo "Generating vkey.json to app/assets/jwt..."
#node -e "const fs = require('fs'); fs.writeFileSync('../app/assets/jwt/circuit-vkey.json', JSON.stringify(Array.from(Uint8Array.from(fs.readFileSync('./target/vk')))));"

echo "Done"
