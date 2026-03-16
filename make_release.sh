#!/usr/bin/env bash
set -euo pipefail

RELEASE_DIR="auroraos_release"
ZIP_NAME="auroraos_personal_release.zip"

rm -rf ${RELEASE_DIR}
mkdir -p ${RELEASE_DIR}
rsync -av --exclude='node_modules' --exclude='.cache' . ${RELEASE_DIR}/

echo "Creating zip ${ZIP_NAME}..."
zip -r ${ZIP_NAME} ${RELEASE_DIR}

sha256sum ${ZIP_NAME} > ${ZIP_NAME}.sha256

echo "Release created: ${ZIP_NAME}"
