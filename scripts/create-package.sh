#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <name>"
    exit 1
fi

package_name=$1
cargo_toml="Cargo.toml"

# Create new Cargo project
cargo new $package_name --vcs=none

echo "Package $package_name created successfully! Add $package_name to your root Cargo.toml file."
