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

# Record every maintained VHS gallery
vhs-all:
    cargo build --release --example readme
    mkdir -p target/vhs
    @for tape in examples/vhs/*.tape; do env -u NO_COLOR vhs "$tape" || exit; done
    just vhs-optimize

# Restore GIF data from LFS pointers before repeating the optimization
vhs-materialize:
    @for tape in examples/vhs/*.tape; do \
        gif="examples/vhs/generated/$(basename "$tape" .tape).gif"; \
        if grep -qF 'version https://git-lfs.github.com/spec/v1' "$gif"; then \
            git lfs smudge < "$gif" > "$gif.materialized" || exit; \
            mv "$gif.materialized" "$gif"; \
        fi; \
    done

# Losslessly optimize existing GIFs with ImageMagick
# Current recordings shrink substantially: Flux 32.4 MB -> 5.0 MB, Bar 9-10 MB -> about 1 MB,
# and Linear 2.4-5.3 MB -> 0.2-0.6 MB
# Keeps the VHS output when optimization would make an image larger
vhs-optimize: vhs-materialize
    @command -v magick >/dev/null 2>&1 || { echo "ImageMagick not found"; exit 1; }
    @for tape in examples/vhs/*.tape; do \
        gif="examples/vhs/generated/$(basename "$tape" .tape).gif"; \
        optimized="${gif%.gif}.optimized.gif"; \
        before=$(wc -c < "$gif" | tr -d ' '); \
        magick "$gif" -coalesce -layers Optimize "$optimized" || exit; \
        after=$(wc -c < "$optimized" | tr -d ' '); \
        if [ "$after" -lt "$before" ]; then \
            mv "$optimized" "$gif"; \
            printf '%s: %s -> %s bytes\n' "$gif" "$before" "$after"; \
        else \
            rm "$optimized"; \
            printf '%s: kept %s bytes (candidate %s)\n' "$gif" "$before" "$after"; \
        fi; \
    done
    just vhs-lfs-clean

# Store generated media as LFS pointers because jj does not run Git clean filters when snapshotting
vhs-lfs-clean:
    @command -v git-lfs >/dev/null 2>&1 || { echo "Git LFS not found"; exit 1; }
    @for tape in examples/vhs/*.tape; do \
        gif="examples/vhs/generated/$(basename "$tape" .tape).gif"; \
        if test "$(head -c 6 "$gif")" = 'GIF89a'; then \
            git lfs clean -- "$gif" < "$gif" > "$gif.pointer" || exit; \
            git lfs pointer --check --file="$gif.pointer" || exit; \
            mv "$gif.pointer" "$gif"; \
        fi; \
    done

# Record one tape, for example: just vhs-tape overview
vhs-tape name:
    cargo build --release --example readme
    mkdir -p target/vhs
    env -u NO_COLOR vhs "examples/vhs/{{ name }}.tape"
    just vhs-lfs-clean

# List the available recording sources.
vhs-list:
    @ls examples/vhs/*.tape

# Pull GIF files from Git LFS (run once after a fresh clone)
lfs-pull:
    @command -v git-lfs >/dev/null 2>&1 || { \
        echo "Git LFS not found. Install with: brew install git-lfs"; exit 1; \
    }
    git lfs pull
