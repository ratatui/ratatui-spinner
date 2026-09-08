# ratatui-spinner development tasks

# List available development commands.
default:
    @just --list

# Build the library in the development profile.
build:
    cargo build

# Build the optimized library without publishing it.
build-release:
    cargo build --release

# Run the combined spinner demonstration.
run:
    cargo run --example spinner

# Run unit tests and documentation examples.
test:
    cargo test --locked --all-features

# Check compilation without producing an executable.
check:
    cargo check --locked --all-targets --all-features

# Format Rust source and documentation examples.
fmt:
    cargo +nightly fmt

# Verify formatting without modifying files.
fmt-check:
    cargo +nightly fmt --check

# Lint Rust code and treat warnings as failures.
clippy:
    cargo clippy --locked --all-targets --all-features -- -D warnings

# Run the standard local development checks.
check-all: fmt-check clippy test doc

# Build API documentation.
doc:
    RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc --locked --no-deps --all-features

# Report available dependency updates (requires cargo-outdated).
outdated:
    cargo outdated

# Inspect package metadata, including its version, with Cargo's standard output.
metadata:
    cargo metadata --no-deps --format-version 1

# Verify the distributable locally without uploading it.
package:
    cargo package --locked

# Changelog generation is local; release/tag creation remains a separate decision.
changelog:
    git-cliff --output CHANGELOG.md

# Preview unreleased changelog entries without changing files.
changelog-preview:
    git-cliff --unreleased

# Run development checks, an optimized build, and package verification.
check-release: check-all build-release package

# VHS recordings are written to examples/vhs/generated and tracked with Git LFS.
vhs-all:
    @for tape in examples/vhs/*.tape; do vhs "$tape" || exit; done

# Record one tape, for example: just vhs-tape spinner-demo.
vhs-tape name:
    vhs "examples/vhs/{{ name }}.tape"

# List the available recording sources.
vhs-list:
    @ls examples/vhs/*.tape

# Pull GIF files from Git LFS (run once after a fresh clone)
lfs-pull:
    @command -v git-lfs >/dev/null 2>&1 || { \
        echo "Git LFS not found. Install with: brew install git-lfs"; exit 1; \
    }
    git lfs pull
