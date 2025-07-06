# Project configuration
name := "adsb"
version := env_var_or_default("VERSION", "v0.2.0")
bindir := "target/release"

# Platform target mappings
# Tier 1: Primary support - tested and recommended
# Tier 2: Secondary support - tested but may have limitations
# Experimental: Cutting-edge platforms - use at your own risk

# Default recipe: build most commonly used platforms
default:
    @echo "Building all Linux platforms..."
    @just build-all-linux

# Build all supported platforms
all-arch: check-deps
    @echo "Building all supported platforms..."
    @just build-all-linux build-all-darwin build-all-windows

# Platform-specific build groups
build-all-linux: check-deps
    @echo "Building all Linux platforms..."
    @just build-linux-386 build-linux-amd64 build-linux-armv6 build-linux-armv7 build-linux-arm64 build-linux-riscv64 build-linux-loongarch

build-all-darwin: check-deps
    @echo "Building all macOS platforms..."
    @just build-universal2-apple-darwin

build-all-windows: check-deps
    @echo "Building all Windows platforms..."
    @just build-windows-386 build-windows-amd64

build-all-arm: check-deps
    @echo "Building all ARM platforms..."
    @just build-linux-armv6 build-linux-armv7 build-linux-arm64

# Parallel build recipes for faster compilation
build-all-linux-parallel: check-deps
    @echo "Building all Linux platforms in parallel..."
    @echo "build-linux-386 build-linux-amd64 build-linux-armv6 build-linux-armv7 build-linux-arm64 build-linux-riscv64 build-linux-loongarch" | tr ' ' '\n' | xargs -n1 -P4 just

build-tier1-parallel: check-deps
    @echo "Building Tier 1 platforms in parallel..."
    @echo "build-linux-386 build-linux-amd64 build-linux-arm64 build-universal2-apple-darwin build-windows-amd64" | tr ' ' '\n' | xargs -n1 -P3 just

# Individual platform builds - macOS/Darwin
build-universal2-apple-darwin:
    @echo "Building macOS Universal2 binary (Intel + Apple Silicon)..."
    @echo "  Using native macOS toolchain with lipo..."
    @rustup target add x86_64-apple-darwin aarch64-apple-darwin
    @echo "  Building Intel binary..."
    @cargo build --target x86_64-apple-darwin --release || (echo "Build failed for x86_64-apple-darwin" && exit 1)
    @echo "  Building Apple Silicon binary..."
    @cargo build --target aarch64-apple-darwin --release || (echo "Build failed for aarch64-apple-darwin" && exit 1)
    @echo "  Creating Universal2 binary with lipo..."
    @mkdir -p {{bindir}}
    @mkdir -p target/universal2-apple-darwin/release
    @lipo -create \
        target/x86_64-apple-darwin/release/{{name}} \
        target/aarch64-apple-darwin/release/{{name}} \
        -output target/universal2-apple-darwin/release/{{name}}
    @cp target/universal2-apple-darwin/release/{{name}} {{bindir}}/{{name}}-{{version}}-universal2-apple-darwin
    @echo "  macOS Universal2 build complete"
    @just verify-build universal2-apple-darwin

# Individual platform builds - Linux
build-linux-386:
    @echo "Building Linux i686..."
    @echo "  Target: i686-unknown-linux-gnu"
    @rustup target add i686-unknown-linux-gnu
    @cargo zigbuild --target i686-unknown-linux-gnu --release || (echo "Build failed for linux-386" && exit 1)
    @mkdir -p {{bindir}}
    @cp target/i686-unknown-linux-gnu/release/{{name}} {{bindir}}/{{name}}-{{version}}-linux-386
    @echo "  Linux i686 build complete"
    @just verify-build linux-386

build-linux-amd64:
    @echo "Building Linux x86_64..."
    @echo "  Target: x86_64-unknown-linux-gnu"
    @rustup target add x86_64-unknown-linux-gnu
    @cargo zigbuild --target x86_64-unknown-linux-gnu --release || (echo "Build failed for linux-amd64" && exit 1)
    @mkdir -p {{bindir}}
    @cp target/x86_64-unknown-linux-gnu/release/{{name}} {{bindir}}/{{name}}-{{version}}-linux-amd64
    @echo "  Linux x86_64 build complete"
    @just verify-build linux-amd64

build-linux-armv6:
    @echo "Building Linux ARMv6 (Raspberry Pi Zero)..."
    @echo "  Target: arm-unknown-linux-gnueabihf"
    @rustup target add arm-unknown-linux-gnueabihf
    @cargo zigbuild --target arm-unknown-linux-gnueabihf --release || (echo "Build failed for linux-armv6" && exit 1)
    @mkdir -p {{bindir}}
    @cp target/arm-unknown-linux-gnueabihf/release/{{name}} {{bindir}}/{{name}}-{{version}}-linux-armv6
    @echo "  Linux ARMv6 build complete"
    @just verify-build linux-armv6

build-linux-armv7:
    @echo "Building Linux ARMv7 (Raspberry Pi 2/3)..."
    @echo "  Target: armv7-unknown-linux-gnueabihf"
    @rustup target add armv7-unknown-linux-gnueabihf
    @cargo zigbuild --target armv7-unknown-linux-gnueabihf --release || (echo "Build failed for linux-armv7" && exit 1)
    @mkdir -p {{bindir}}
    @cp target/armv7-unknown-linux-gnueabihf/release/{{name}} {{bindir}}/{{name}}-{{version}}-linux-armv7
    @echo "  Linux ARMv7 build complete"
    @just verify-build linux-armv7

build-linux-arm64:
    @echo "Building Linux ARM64 (Raspberry Pi 4+, RK3588/RK3588S, RK3566/RK3568, RK3399)..."
    @echo "  Target: aarch64-unknown-linux-gnu"
    @rustup target add aarch64-unknown-linux-gnu
    @cargo zigbuild --target aarch64-unknown-linux-gnu --release || (echo "Build failed for linux-arm64" && exit 1)
    @mkdir -p {{bindir}}
    @cp target/aarch64-unknown-linux-gnu/release/{{name}} {{bindir}}/{{name}}-{{version}}-linux-arm64
    @echo "  Linux ARM64 build complete"
    @just verify-build linux-arm64

build-linux-riscv64:
    @echo "Building Linux RISC-V 64-bit..."
    @echo "  Target: riscv64gc-unknown-linux-gnu"
    @rustup target add riscv64gc-unknown-linux-gnu
    @cargo zigbuild --target riscv64gc-unknown-linux-gnu --release || (echo "Build failed for linux-riscv64" && exit 1)
    @mkdir -p {{bindir}}
    @cp target/riscv64gc-unknown-linux-gnu/release/{{name}} {{bindir}}/{{name}}-{{version}}-linux-riscv64
    @echo "  Linux RISC-V 64-bit build complete"
    @just verify-build linux-riscv64

build-linux-loongarch:
    @echo "Building Linux LoongArch 64-bit..."
    @echo "  Target: loongarch64-unknown-linux-gnu"
    @rustup target add loongarch64-unknown-linux-gnu
    @cargo zigbuild --target loongarch64-unknown-linux-gnu --release || (echo "Build failed for linux-loongarch" && exit 1)
    @mkdir -p {{bindir}}
    @cp target/loongarch64-unknown-linux-gnu/release/{{name}} {{bindir}}/{{name}}-{{version}}-linux-loongarch
    @echo "  Linux LoongArch 64-bit build complete"
    @just verify-build linux-loongarch


# Individual platform builds - Windows
build-windows-386:
    @echo "Building Windows i686..."
    @echo "  Target: i686-pc-windows-gnu"
    @rustup target add i686-pc-windows-gnu
    @export CARGO_TARGET_I686_PC_WINDOWS_GNU_LINKER="i686-w64-mingw32-gcc"
    @cargo zigbuild --target i686-pc-windows-gnu --release || (echo "Build failed for windows-386" && exit 1)
    @mkdir -p {{bindir}}
    @cp target/i686-pc-windows-gnu/release/{{name}}.exe {{bindir}}/{{name}}-{{version}}-windows-386.exe
    @echo "  Windows i686 build complete"
    @just verify-build windows-386

build-windows-amd64:
    @echo "Building Windows x86_64..."
    @echo "  Target: x86_64-pc-windows-gnu"
    @rustup target add x86_64-pc-windows-gnu
    @cargo build --target x86_64-pc-windows-gnu --release || (echo "Build failed for windows-amd64" && exit 1)
    @mkdir -p {{bindir}}
    @cp target/x86_64-pc-windows-gnu/release/{{name}}.exe {{bindir}}/{{name}}-{{version}}-windows-amd64.exe
    @echo "  Windows x86_64 build complete"
    @just verify-build windows-amd64

# Utility recipes
check-deps:
    @echo "Checking build dependencies..."
    @command -v cargo >/dev/null || (echo "cargo not found - install Rust first" && exit 1)
    @command -v just >/dev/null || (echo "just not found - run 'cargo install just'" && exit 1)
    @cargo-zigbuild --version >/dev/null 2>&1 || (echo "cargo-zigbuild not found - run 'cargo install cargo-zigbuild'" && exit 1)
    @echo "All dependencies satisfied"

verify-build target:
    #!/bin/bash
    set -eu
    echo "Verifying {{target}} build..."

    # Determine file extension
    if echo "{{target}}" | grep -q "windows"; then
        ext=".exe"
    else
        ext=""
    fi

    filepath="{{bindir}}/{{name}}-{{version}}-{{target}}${ext}"

    if [ -f "${filepath}" ]; then
        echo "  Binary exists: ${filepath}"
        size=$(stat -f%z "${filepath}" 2>/dev/null || stat -c%s "${filepath}" 2>/dev/null || echo "unknown")
        echo "  Size: ${size} bytes"
        if command -v file >/dev/null; then
            echo "  Type: $(file "${filepath}")"
        fi
    else
        echo "  Binary missing: ${filepath}"
        exit 1
    fi

# Development and testing recipes
dev-build:
    @echo "Building for development (current platform)..."
    @cargo build
    @echo "Development build complete"

dev-run *ARGS:
    @echo "Running development build..."
    @cargo run -- {{ARGS}}

test:
    @echo "Running tests..."
    @cargo test
    @echo "All tests passed"

fmt:
    @echo "Formatting code..."
    @cargo fmt
    @echo "Code formatted"

clippy:
    @echo "Running clippy lints..."
    @cargo clippy -- -D warnings
    @echo "No clippy warnings"

check: fmt clippy test
    @echo "All checks passed"

# Cleaning recipes
clean:
    @echo "Cleaning build artifacts..."
    @cargo clean
    @rm -rf {{bindir}}
    @echo "Clean complete"

clean-target target:
    @echo "Cleaning {{target}} artifacts..."
    @rm -f {{bindir}}/{{name}}-*-{{target}}*
    @echo "{{target}} artifacts cleaned"

clean-releases:
    @echo "Cleaning release artifacts (keeping source builds)..."
    @find {{bindir}} -name "*.gz" -delete 2>/dev/null || true
    @find {{bindir}} -name "*.sha256" -delete 2>/dev/null || true
    @echo "Release artifacts cleaned"

# Release management
release: checksum
    @echo ""
    @echo "Release {{version}} created successfully!"
    @echo "Location: {{bindir}}/"
    @echo ""
    @echo "Release Summary:"
    @ls -la {{bindir}}/ | grep -E "({{name}}-{{version}}|\.gz$|\.sha256$)" || true
    @echo ""

compress: all-arch
    @echo "Compressing release binaries..."
    #!/bin/bash
    set -eu
    cd {{bindir}}
    for file in $(find . -name "{{name}}-{{version}}-*" -not -name "*.gz" -not -name "*.sha256" -type f); do
    if [ ! -f "${file}.gz" ]; then
    echo "  ${file#./} → ${file#./}.gz"
    gzip -c "${file#./}" > "${file#./}.gz"
    fi
    done
    echo "Compression complete"

checksum: compress
    @echo "Generating SHA256 checksums..."
    #!/bin/bash
    set -eu
    cd {{bindir}}
    for file in $(find . -name "{{name}}-{{version}}-*" -type f -not -name "*.sha256"); do
    if [ ! -f "${file}.sha256" ]; then
    echo "  ${file#./} → ${file#./}.sha256"
    if command -v shasum >/dev/null; then
    shasum -a 256 "${file#./}" > "${file#./}.sha256"
    elif command -v sha256sum >/dev/null; then
    sha256sum "${file#./}" > "${file#./}.sha256"
    else
    echo "No SHA256 utility found"
    exit 1
    fi
    fi
    done
    echo "Checksums generated"

# Information and help
list-targets:
    @echo "Available build targets:"
    @echo ""
    @echo "Tier 1 (Primary Support):"
    @echo "  linux-amd64             - Linux x86_64"
    @echo "  linux-arm64             - Linux ARM64"
    @echo "  universal2-apple-darwin - macOS Universal2 (Intel + Apple Silicon)"
    @echo "  windows-amd64           - Windows x86_64"
    @echo ""
    @echo "Tier 2 (Secondary Support):"
    @echo "  linux-386               - Linux i686"
    @echo "  linux-armv6/armv7       - ARM variants"
    @echo "  linux-riscv64           - RISC-V 64-bit"
    @echo "  linux-loongarch         - LoongArch 64-bit"
    @echo "  windows-386             - Windows i686"
    @echo "  windows-arm64           - Windows ARM64"
    @echo ""

help:
    @echo "{{name}} Build System"
    @echo ""
    @echo "Build Commands:"
    @echo "  just                     - Build all Linux platforms (recommended)"
    @echo "  just all-arch            - Build all supported platforms"
    @echo "  just build-all-linux     - Build all Linux platforms"
    @echo "  just build-all-darwin    - Build all macOS platforms"
    @echo "  just build-all-windows   - Build all Windows platforms"
    @echo "  just build-all-arm       - Build all ARM platforms"
    @echo ""
    @echo "Parallel Build Commands:"
    @echo "  just build-tier1-parallel     - Build Tier 1 platforms in parallel"
    @echo "  just build-all-linux-parallel - Build Linux platforms in parallel"
    @echo ""
    @echo "Individual Targets:"
    @echo "  just build-<platform>    - Build specific platform"
    @echo "  just list-targets        - Show all available platforms"
    @echo ""
    @echo "Development Commands:"
    @echo "  just dev-build           - Build for development"
    @echo "  just dev-run [ARGS]      - Run development build"
    @echo "  just test                - Run tests"
    @echo "  just check               - Run fmt + clippy + test"
    @echo ""
    @echo "Release Commands:"
    @echo "  just release             - Create full release (build + compress + checksum)"
    @echo "  just compress            - Compress binaries with gzip"
    @echo "  just checksum            - Generate SHA256 checksums"
    @echo ""
    @echo "Cleanup Commands:"
    @echo "  just clean               - Clean all build artifacts"
    @echo "  just clean-target TARGET - Clean specific target artifacts"
    @echo "  just clean-releases      - Clean compressed files and checksums"
    @echo ""
    @echo "Utility Commands:"
    @echo "  just check-deps          - Check build dependencies"
    @echo "  just verify-build TARGET - Verify specific build"
    @echo "  just help                - Show this help message"
    @echo ""
    @echo "Examples:"
    @echo "  just build-linux-amd64              # Build Linux x64"
    @echo "  just build-tier1-parallel           # Fast build of main platforms"
    @echo "  just dev-run --help                 # Run with arguments"
    @echo "  VERSION=v1.0.0 just release         # Create tagged release"
    @echo ""
    @echo "More info: https://github.com/casey/just"
