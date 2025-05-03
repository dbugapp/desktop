#!/bin/bash

set -a
source .env
set +a

./scripts/build-macos.sh
./scripts/sign-os.sh target/release/macos/dbug.app/
./scripts/package-macos.sh
./scripts/sign-os.sh target/release/macos/dbug.dmg
sleep 20
./scripts/staple.sh target/release/macos/dbug.dmg
