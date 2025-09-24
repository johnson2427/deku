#!/bin/bash
# Set the version you want
VERSION="v0.42.0"

# Download the tarball
curl -LO https://github.com/paritytech/substrate-contracts-node/releases/download/$VERSION/substrate-contracts-node-linux.tar.gz

# Extract it
tar -xzf substrate-contracts-node-linux.tar.gz

# Move it to somewhere in your PATH
sudo mv substrate-contracts-node-linux/substrate-contracts-node /usr/local/bin/

# Make sure it's executable
chmod +x /usr/local/bin/substrate-contracts-node

rm -rf substrate-contracts-node-linux
rm -rf substrate-contracts-node-linux.tar.gz

# Test it
substrate-contracts-node --version
