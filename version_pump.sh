#!/bin/bash

CONFIGS=(Cargo.toml python/Cargo.toml)

version=$(git describe --tags --abbrev=0)
version=${version/[a-zA-Z]/}
for config in "${CONFIGS[@]}"
do
  sed -i "s/^\(version = \).*/\1\"$version\"/" "$config"
done
