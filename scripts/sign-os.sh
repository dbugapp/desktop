#!/bin/bash
# usage: script/sign <file>
#
# Signs macOS binaries using codesign, notarizes macOS zip archives using notarytool, and signs
# Windows EXE and MSI files using osslsigncode.
#
set -e

sign_windows() {
  if [ -z "$CERT_FILE" ]; then
    echo "skipping Windows code-signing; CERT_FILE not set" >&2
    return 0
  fi

  if [ ! -f "$CERT_FILE" ]; then
    echo "error Windows code-signing; file '$CERT_FILE' not found" >&2
    return 1
  fi

  if [ -z "$KEY_FILE" ]; then
    echo "error Windows code-signing; no value for KEY_FILE" >&2
    return 1
  fi

  osslsigncode sign -n "Dbug App" \
    -certs "$CERT_FILE" -key "$KEY_FILE" \
    -in "$1" -out "$1"~

  mv "$1"~ "$1"
}

sign_macos() {
  if [ -z "$APPLE_DEVELOPER_ID" ]; then
    echo "skipping macOS code-signing; APPLE_DEVELOPER_ID not set" >&2
    return 0
  fi

  if [[ $1 == *.dmg ]]; then
    echo "Running notarytool submit for $1"
    echo "Apple ID: ${APPLE_ID?}"
    echo "Team ID: ${APPLE_DEVELOPER_ID?}"
    xcrun notarytool submit "$1" --apple-id "${APPLE_ID?}" --team-id "${APPLE_DEVELOPER_ID?}" --password "${APPLE_ID_PASSWORD?}"
  else
    echo "Running codesign for $1"
    echo "Using Developer ID: ${APPLE_DEVELOPER_ID?}"
    codesign --timestamp --options=runtime -s "${APPLE_DEVELOPER_ID?}" -v "$1"
  fi

}

if [ $# -eq 0 ]; then
  echo "usage: script/sign <file>" >&2
  exit 1
fi

platform="$(uname -s)"

for input_file; do
  case "$input_file" in
  *.exe | *.msi )
    sign_windows "$input_file"
    ;;
  * )
    if [ "$platform" = "Darwin" ]; then
      sign_macos "$input_file"
    else
      printf "warning: don't know how to sign %s on %s\n" "$1", "$platform" >&2
    fi
    ;;
  esac
done
