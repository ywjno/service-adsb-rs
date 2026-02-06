# =============================================================================
# service-adsb-rs - cross-platform build
# =============================================================================
#
#   just dev               debug build (current platform)
#   just build-tier1       Tier 1 targets
#   just -j build-tier1    Tier 1 targets in parallel (just 1.6+)
#   just release           Tier 1 release: build -> gzip -> sha256
#   just list              all available targets
#
# Output: ./dist/  (survives `cargo clean`)
# =============================================================================

name    := "adsb"
version := env_var_or_default("VERSION", "v0.3.0")
outdir  := "dist"

# ---------------------------------------------------------------------------
# default
# ---------------------------------------------------------------------------

default: list

# ---------------------------------------------------------------------------
# Tier 1 - primary support
# ---------------------------------------------------------------------------

build-tier1: build-linux-amd64 build-linux-arm64 build-darwin-universal2 build-windows-amd64

# Tier 1 minus macOS (for Linux, where macOS cross-compile is unavailable)
build-tier1-no-darwin: build-linux-amd64 build-linux-arm64 build-windows-amd64

build-linux-amd64: check-deps
    @just _build x86_64-unknown-linux-gnu linux-amd64

build-linux-arm64: check-deps
    @just _build aarch64-unknown-linux-gnu linux-arm64

build-darwin-universal2: check-deps
    @just _build-universal2

build-windows-amd64: check-deps
    @just _build x86_64-pc-windows-gnu windows-amd64

# ---------------------------------------------------------------------------
# Tier 2 - secondary support
# ---------------------------------------------------------------------------

build-tier2: build-linux-386 build-linux-armv6 build-linux-armv7 build-linux-riscv64 build-linux-loongarch build-windows-386

build-linux-386: check-deps
    @just _build i686-unknown-linux-gnu linux-386

build-linux-armv6: check-deps
    @just _build arm-unknown-linux-gnueabihf linux-armv6

build-linux-armv7: check-deps
    @just _build armv7-unknown-linux-gnueabihf linux-armv7

build-linux-riscv64: check-deps
    @just _build riscv64gc-unknown-linux-gnu linux-riscv64

build-linux-loongarch: check-deps
    @just _build loongarch64-unknown-linux-gnu linux-loongarch

build-windows-386: check-deps
    @just _build i686-pc-windows-gnu windows-386

# ---------------------------------------------------------------------------
# musl - static binaries for Docker
# ---------------------------------------------------------------------------

build-docker: build-linux-amd64-musl build-linux-arm64-musl build-linux-armv6-musl build-linux-armv7-musl build-linux-386-musl build-linux-riscv64-musl

build-linux-amd64-musl: check-deps
    @just _build x86_64-unknown-linux-musl linux-amd64-musl

build-linux-arm64-musl: check-deps
    @just _build aarch64-unknown-linux-musl linux-arm64-musl

build-linux-armv6-musl: check-deps
    @just _build arm-unknown-linux-musleabihf linux-armv6-musl

build-linux-armv7-musl: check-deps
    @just _build armv7-unknown-linux-musleabihf linux-armv7-musl

build-linux-386-musl: check-deps
    @just _build i686-unknown-linux-musl linux-386-musl

build-linux-riscv64-musl: check-deps
    @just _build riscv64gc-unknown-linux-musl linux-riscv64-musl

# ---------------------------------------------------------------------------
# Combo
# ---------------------------------------------------------------------------

build-all: build-tier1 build-tier2
build-arm: build-linux-armv6 build-linux-armv7 build-linux-arm64

# ---------------------------------------------------------------------------
# Release
# ---------------------------------------------------------------------------

release: build-tier1
    @just _compress
    @just _checksum
    @echo ""
    @echo "  release {{version}} -> {{outdir}}/"
    @ls -lh {{outdir}}/

release-all: build-all
    @just _compress
    @just _checksum
    @echo ""
    @echo "  release {{version}} (all) -> {{outdir}}/"
    @ls -lh {{outdir}}/

# ---------------------------------------------------------------------------
# Development
# ---------------------------------------------------------------------------

dev:
    @cargo build
    @echo "-> target/debug/{{name}}"

run *ARGS:
    @cargo run -- {{ARGS}}

test:
    @cargo test --all-features

fmt:
    @cargo fmt --all

lint:
    @cargo clippy --all-targets --all-features -- -D warnings

doc:
    @cargo doc --no-deps --all-features

check: fmt lint test doc

# ---------------------------------------------------------------------------
# Verification
# ---------------------------------------------------------------------------

verify-tier1:
    @just _verify linux-amd64 linux-arm64 darwin-universal2 windows-amd64

verify-tier2:
    @just _verify linux-386 linux-armv6 linux-armv7 linux-riscv64 linux-loongarch windows-386

verify-all: verify-tier1 verify-tier2

# ---------------------------------------------------------------------------
# Cleanup
# ---------------------------------------------------------------------------

clean:
    @cargo clean
    @rm -rf {{outdir}}

clean-dist:
    @rm -rf {{outdir}}

# ---------------------------------------------------------------------------
# Info
# ---------------------------------------------------------------------------

list:
    @echo ""
    @echo " {{name}} {{version}}"
    @echo ""
    @echo " Tier 1 (primary)"
    @echo "   build-linux-amd64          x86_64 Linux"
    @echo "   build-linux-arm64          ARM64 Linux (RPi 4+, etc.)"
    @echo "   build-darwin-universal2    macOS Intel + Apple Silicon"
    @echo "   build-windows-amd64        x86_64 Windows"
    @echo ""
    @echo " Tier 2 (secondary)"
    @echo "   build-linux-386            i686 Linux"
    @echo "   build-linux-armv6          ARMv6 Linux (RPi Zero/1)"
    @echo "   build-linux-armv7          ARMv7 Linux (RPi 2/3)"
    @echo "   build-linux-riscv64        RISC-V 64 Linux"
    @echo "   build-linux-loongarch      LoongArch 64 Linux"
    @echo "   build-windows-386          i686 Windows"
    @echo ""
    @echo " musl / Docker"
    @echo "   build-linux-<arch>-musl    static musl variants"
    @echo "   build-docker               all musl targets"
    @echo ""
    @echo " Groups"
    @echo "   build-tier1                Tier 1"
    @echo "   build-tier1-no-darwin      Tier 1 without macOS (in Linux)"
    @echo "   build-tier2                Tier 2"
    @echo "   build-all                  Tier 1 + Tier 2"
    @echo "   build-arm                  ARMv6 + ARMv7 + ARM64"
    @echo ""
    @echo " Release"
    @echo "   release                    Tier 1 -> gzip -> sha256"
    @echo "   release-all                Tier 1+2 -> gzip -> sha256"
    @echo ""
    @echo " Dev"
    @echo "   dev                        debug build"
    @echo "   run [ARGS]                 debug run"
    @echo "   test / check               test / fmt + lint + test + doc"
    @echo ""
    @echo " Verify"
    @echo "   verify-tier1 / verify-tier2 / verify-all"
    @echo ""
    @echo " Tips"
    @echo "   just -j build-tier1              parallel build"
    @echo "   VERSION=v1.0.0 just release      tagged release"
    @echo ""

# =============================================================================
# Internals
# =============================================================================

check-deps:
    @command -v cargo          >/dev/null 2>&1 || { echo "[!!] cargo not found"; exit 1; }
    @command -v cargo-zigbuild >/dev/null 2>&1 || { echo "[!!] cargo-zigbuild missing - cargo install cargo-zigbuild"; exit 1; }
    @command -v zig            >/dev/null 2>&1 || { echo "[!!] zig missing - https://ziglang.org/download/"; exit 1; }

# Cross-compile and copy to dist/
_build rust_target output_name:
    #!/bin/bash
    set -euo pipefail

    rustup target add "{{rust_target}}" 2>/dev/null || true
    cargo zigbuild --target "{{rust_target}}" --release

    mkdir -p "{{outdir}}"

    if [[ "{{rust_target}}" == *windows* ]]; then
        src="target/{{rust_target}}/release/{{name}}.exe"
        dst="{{outdir}}/{{name}}-{{version}}-{{output_name}}.exe"
    else
        src="target/{{rust_target}}/release/{{name}}"
        dst="{{outdir}}/{{name}}-{{version}}-{{output_name}}"
    fi

    cp "$src" "$dst"
    size=$(stat -f%z "$dst" 2>/dev/null || stat -c%s "$dst")
    echo "  [ok] $(basename "$dst") ($size bytes)"

# macOS Universal2: build both slices, combine with lipo.
_build-universal2:
    #!/bin/bash
    set -euo pipefail

    # Requires macOS SDK: either run natively on macOS, or set SDKROOT
    # to a downloaded SDK (bypasses xcrun).  CI uses macos-latest runner.
    if [[ "$(uname -s)" != "Darwin" ]] && [ -z "${SDKROOT:-}" ]; then
        echo "[!!] darwin-universal2 needs macOS or SDKROOT"
        echo "  option 1: run on macOS"
        echo "  option 2: set SDKROOT to a macOS SDK path"
        echo "  CI: release.yml already uses macos-latest for this target"
        echo "  local Linux: use 'just build-tier1-no-darwin' to skip macOS"
        exit 1
    fi

    rustup target add x86_64-apple-darwin aarch64-apple-darwin 2>/dev/null || true

    cargo zigbuild --target x86_64-apple-darwin  --release
    cargo zigbuild --target aarch64-apple-darwin --release

    mkdir -p "{{outdir}}"
    dst="{{outdir}}/{{name}}-{{version}}-darwin-universal2"

    lipo -create \
        target/x86_64-apple-darwin/release/{{name}} \
        target/aarch64-apple-darwin/release/{{name}} \
        -output "$dst"

    size=$(stat -f%z "$dst" 2>/dev/null || stat -c%s "$dst")
    echo "  [ok] $(basename "$dst") ($size bytes)"

# gzip all release binaries (keeps originals)
_compress:
    #!/bin/bash
    set -euo pipefail
    shopt -s nullglob

    for f in {{outdir}}/{{name}}-{{version}}-*; do
        [[ "$f" == *.gz     ]] && continue
        [[ "$f" == *.sha256 ]] && continue
        gzip -kf "$f"
        echo "  [+] $(basename "${f}.gz")"
    done
    echo "  [ok] compressed"

# Single sha256 manifest for all release files
_checksum:
    #!/bin/bash
    set -euo pipefail
    shopt -s nullglob

    cd "{{outdir}}"
    manifest="{{name}}-{{version}}.sha256"
    > "$manifest"

    for f in {{name}}-{{version}}-*; do
        [[ "$f" == *.sha256 ]] && continue
        if   command -v sha256sum >/dev/null 2>&1; then sha256sum    "$f"
        elif command -v shasum    >/dev/null 2>&1; then shasum -a 256 "$f"
        else echo "[!!] no sha256sum or shasum"; exit 1
        fi >> "$manifest"
    done
    echo "  [ok] checksums -> $manifest"

# Verify built artifacts exist in dist/
_verify *names:
    #!/bin/bash
    set -euo pipefail
    ok=0
    miss=0

    for n in {{names}}; do
        found=0
        for ext in "" ".exe"; do
            f="{{outdir}}/{{name}}-{{version}}-${n}${ext}"
            if [ -f "$f" ]; then
                size=$(stat -f%z "$f" 2>/dev/null || stat -c%s "$f")
                printf "  [ok]  %-42s %s bytes\n" "$(basename "$f")" "$size"
                found=1
                ok=$((ok + 1))
                break
            fi
        done
        [ $found -eq 0 ] && { printf "  [!!]  %-42s missing\n" "$n"; miss=$((miss + 1)); }
    done

    echo ""
    [ $miss -gt 0 ] && { echo "  $ok ok, $miss missing"; exit 1; }
    echo "  [ok] all $ok present"
