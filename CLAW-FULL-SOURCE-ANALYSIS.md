# CLAW-FULL-SOURCE-ANALYSIS.md
# PROJECT: CLAW - Pocket Assistant Without Limits
# VERSION: 16.0.0-omega
# ARCHITECTURE: 8-layer modular
# PLATFORM: Android 16 (API 36)
# SECURITY: Zero-knowledge, end-to-end encrypted
# COMPILATION TARGET: aarch64-linux-android
# RUST VERSION: 1.75+
# KOTLIN VERSION: 1.9+
# PYTHON VERSION: 3.11+

## FILE: INSTALL.md
```markdown
# CLAW - Installation Guide

## PRAXIS (Current Resources) - 30 mins

### Prerequisites
- Xiaomi 14T Pro with Android 16
- Root: Magisk Delta + LSPosed (Zygisk enabled)
- Shizuku installed and activated
- Termux or ADB shell access

### Step 1: Clone Repository
```bash
git clone https://github.com/claw-omega/claw-project.git
cd claw-project
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
pip install -r src/python/requirements.txt
sdkmanager --install "ndk;26.1.10909125"
./src/bash/generate_enclave_keys.sh
chmod 700 /data/claw
chown root:root /data/claw
cd src/rust/decision_engine
cargo build --release --target aarch64-linux-android
cp target/release/libdecision_engine.so ../../android/app/src/main/jniLibs/
cd ../../android
./gradlew assembleDebug
adb install app/build/outputs/apk/debug/app-debug.apk
adb shell sh /data/local/claw/init_claw.sh
```

## Nota
Este archivo conserva el bloque técnico completo compartido por el usuario en el turno previo para referencia y versionado dentro del repositorio.
