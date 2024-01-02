#!/usr/bin/env bash
set -euxo pipefail

# shellcheck disable=SC2016
MISE_VERSION=$MISE_VERSION \
  MISE_CHECKSUM_LINUX_X86_64=$(grep "mise-v.*linux-x64.tar.gz" "$RELEASE_DIR/$MISE_VERSION/SHASUMS256.txt") \
  MISE_CHECKSUM_LINUX_X86_64_MUSL=$(grep "mise-v.*linux-x64-musl.tar.gz" "$RELEASE_DIR/$MISE_VERSION/SHASUMS256.txt") \
  MISE_CHECKSUM_LINUX_ARM64=$(grep "mise-v.*linux-arm64.tar.gz" "$RELEASE_DIR/$MISE_VERSION/SHASUMS256.txt") \
  MISE_CHECKSUM_LINUX_ARM64_MUSL=$(grep "mise-v.*linux-arm64-musl.tar.gz" "$RELEASE_DIR/$MISE_VERSION/SHASUMS256.txt") \
  MISE_CHECKSUM_LINUX_ARMV6=$(grep "mise-v.*linux-armv6.tar.gz" "$RELEASE_DIR/$MISE_VERSION/SHASUMS256.txt") \
  MISE_CHECKSUM_LINUX_ARMV6_MUSL=$(grep "mise-v.*linux-armv6-musl.tar.gz" "$RELEASE_DIR/$MISE_VERSION/SHASUMS256.txt") \
  MISE_CHECKSUM_LINUX_ARMV7=$(grep "mise-v.*linux-armv7.tar.gz" "$RELEASE_DIR/$MISE_VERSION/SHASUMS256.txt") \
  MISE_CHECKSUM_LINUX_ARMV7_MUSL=$(grep "mise-v.*linux-armv7-musl.tar.gz" "$RELEASE_DIR/$MISE_VERSION/SHASUMS256.txt") \
  MISE_CHECKSUM_MACOS_X86_64=$(grep "mise-v.*macos-x64.tar.gz" "$RELEASE_DIR/$MISE_VERSION/SHASUMS256.txt") \
  MISE_CHECKSUM_MACOS_ARM64=$(grep "mise-v.*macos-arm64.tar.gz" "$RELEASE_DIR/$MISE_VERSION/SHASUMS256.txt") \
  envsubst '$MISE_VERSION,$MISE_CHECKSUM_LINUX_X86_64,$MISE_CHECKSUM_LINUX_ARM64,$MISE_CHECKSUM_MACOS_X86_64,$MISE_CHECKSUM_MACOS_ARM64' \
  <"$BASE_DIR/packaging/standalone/install.envsubst"
