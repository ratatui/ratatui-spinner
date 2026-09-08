# tui-spinner — task runner
# Install just:      cargo install just
# Install git-cliff: cargo install git-cliff
# Install vhs:       brew install vhs  OR  go install github.com/charmbracelet/vhs@latest
# Usage: just <task>
# ── Default ───────────────────────────────────────────────────────────────────

default:
    @just --list

# ── Tool checks ───────────────────────────────────────────────────────────────

_check-git-cliff:
    @command -v git-cliff >/dev/null 2>&1 || { \
        echo "❌ git-cliff not found. Install with: cargo install git-cliff"; exit 1; \
    }

# Check nu (nushell) is available
_check-nu:
    @command -v nu >/dev/null 2>&1 || { \
        echo "❌ nu (nushell) not found. Install: https://www.nushell.sh"; exit 1; \
    }

_check-vhs:
    @command -v vhs >/dev/null 2>&1 || { \
        echo "❌ vhs not found."; \
        echo "   macOS:      brew install vhs"; \
        echo "   Any:        go install github.com/charmbracelet/vhs@latest"; \
        exit 1; \
    }

# Install all recommended development tools
install-tools:
    @echo "Installing development tools…"
    @command -v git-cliff >/dev/null 2>&1 || cargo install git-cliff --locked
    @command -v nu >/dev/null 2>&1 || cargo install nu --locked
    @echo "✅ All tools installed!"

# ── Build ─────────────────────────────────────────────────────────────────────

# Build the library (dev)
build:
    cargo build

# Build release version
build-release:
    cargo build --release

# ── Run ───────────────────────────────────────────────────────────────────────

# Run the main spinner demo example
run:
    cargo run --example spinner

# ── Test ──────────────────────────────────────────────────────────────────────

# Run the Rust test suite
test:
    cargo test --all-features

# Run Nu script tests
test-nu: _check-nu
    nu scripts/tests/run_all.nu

# Run both Rust and Nu tests
test-all-nu: test test-nu
    @echo "✅ All Rust and Nu tests passed!"

# ── Code quality ──────────────────────────────────────────────────────────────

# Check without building
check:
    cargo check

# Format all code
fmt:
    cargo fmt

# Check formatting without modifying files
fmt-check:
    cargo fmt --check

# Run clippy
clippy:
    cargo clippy --all-features -- -D warnings

# Run all quality checks (format, clippy, test, nu) — must pass before a release.

# Auto-formats first, then verifies no changes remain (catches unstaged format diffs).
check-all: fmt clippy test test-nu
    @echo "🔍 Verifying formatting is clean…"
    cargo fmt --check
    @echo "✅ All checks passed!"

# Full pre-release quality gate — everything in check-all plus a release build.
check-release: check-all build-release
    @echo "✅ Release quality gate passed (fmt + clippy + test + nu + release build)!"

# ── VHS Demo GIFs ─────────────────────────────────────────────────────────────

VHS_DIR := "examples/vhs"
VHS_GENERATED := "examples/vhs/generated"

# Generate all VHS demo GIFs
vhs-all: _check-vhs
    @mkdir -p {{ VHS_GENERATED }}
    @echo "=== tui-spinner VHS Tapes ==="
    @for tape in {{ VHS_DIR }}/*.tape; do \
        echo "▶  $$tape"; \
        vhs "$$tape" || echo "❌ Failed: $$tape"; \
    done
    @echo "✅ Demo GIFs generated → {{ VHS_GENERATED }}/"

# Render a single tape by name, e.g.: just vhs-tape spinner-demo
vhs-tape name: _check-vhs
    @if [ -f "{{ VHS_DIR }}/{{ name }}.tape" ]; then \
        echo "▶  {{ VHS_DIR }}/{{ name }}.tape"; \
        vhs "{{ VHS_DIR }}/{{ name }}.tape" && echo "✅ Done."; \
    else \
        echo "❌ Tape not found: {{ name }}.tape"; \
        echo ""; \
        just vhs-list; \
        exit 1; \
    fi

# List all available VHS tapes and any already-generated GIFs
vhs-list:
    @echo "Tapes  →  {{ VHS_DIR }}/"
    @ls {{ VHS_DIR }}/*.tape 2>/dev/null | sed 's|.*/||; s|\.tape||' | sed 's/^/  /' || echo "  (none)"
    @echo ""
    @echo "Generated  →  {{ VHS_GENERATED }}/"
    @ls {{ VHS_GENERATED }}/*.gif 2>/dev/null | sed 's|.*/||' | sed 's/^/  /' || echo "  (none yet)"

# Pull GIF files from Git LFS (run once after a fresh clone)
lfs-pull:
    @command -v git-lfs >/dev/null 2>&1 || { \
        echo "❌ git-lfs not found. Install with: brew install git-lfs"; exit 1; \
    }
    git lfs pull
    @echo "✅ LFS objects pulled."

# ── Documentation ─────────────────────────────────────────────────────────────

# Generate and open docs in browser
doc:
    cargo doc --no-deps --open

# ── Changelog ─────────────────────────────────────────────────────────────────

# Regenerate the full CHANGELOG.md from all tags
changelog: _check-git-cliff
    @echo "Generating full changelog…"
    git-cliff --output CHANGELOG.md
    @echo "✅ CHANGELOG.md updated."

# Prepend only unreleased commits to CHANGELOG.md
changelog-unreleased: _check-git-cliff
    git-cliff --unreleased --prepend CHANGELOG.md
    @echo "✅ Unreleased changes prepended."

# Preview changelog for the next release without writing the file
changelog-preview: _check-git-cliff
    @git-cliff --unreleased

# ── Version bump ──────────────────────────────────────────────────────────────

# Validate that a version string will produce a valid vX.Y.Z tag.
validate-tag version: _check-nu
    @nu scripts/ci/validate_tag.nu "v{{ version }}" 2>&1 >/dev/null

# Fail fast if the requested version is the same as the current one.
_check-version-changed version: _check-nu
    #!/usr/bin/env sh
    current=$(nu scripts/version.nu)
    if [ "$current" = "{{ version }}" ]; then
        echo "❌ Version {{ version }} is already the current version. Nothing to bump."
        exit 1
    fi
    echo "✅ Version will change: $current → {{ version }}"

# Bump the version, regenerate Cargo.lock + CHANGELOG.md, commit and tag.

# Validation runs first (cheap), quality gate runs second (expensive).
bump version: (validate-tag version) (_check-version-changed version) check-release _check-git-cliff
    nu scripts/bump_version.nu --yes {{ version }}

# ── Publish (crates.io) ───────────────────────────────────────────────────────

# Run the full pre-publish readiness check (fmt, clippy, tests, docs, dry-run)
check-publish: _check-nu
    nu scripts/check_publish.nu

# Dry-run publish
publish-dry: check-all
    cargo publish --dry-run

# Show what would be released without making any changes
release-preview: _check-git-cliff
    @echo "Current version: $(just version)"
    @echo ""
    @echo "Unreleased commits:"
    @git-cliff --unreleased
    @echo ""
    @echo "Published crate:  tui-spinner"

# ── Housekeeping ──────────────────────────────────────────────────────────────

# Remove build artifacts
clean:
    cargo clean

# Update all dependencies (Cargo.lock only)
update:
    cargo update

# Show outdated dependencies (requires cargo-outdated)
outdated:
    cargo outdated

# Show the current crate version
version: _check-nu
    @nu scripts/version.nu

# Show all configured remotes
remotes:
    @git remote -v

# Pull the current branch from GitHub (origin)
pull:
    git pull origin main
