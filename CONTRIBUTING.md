# Contributing to ratatui-spinner

Contributions should keep the spinner widgets predictable, visually useful, and compatible with
Ratatui's rendering model. This guide covers the checks and repository conventions used by the
project.

## Development setup

The crate declares Rust 1.88 as its minimum supported version. The development checks use the
current stable toolchain for Clippy and tests, plus nightly Rust for formatting and documentation.
Install both toolchains and the required components with rustup:

```sh
rustup toolchain install stable --component clippy
rustup toolchain install nightly --component rustfmt
```

The repository's common commands are defined in the `justfile`. Install
[`just`](https://github.com/casey/just) if you want to use those shortcuts; every CI command is also
listed below in its underlying Cargo form.

Generated gallery GIFs use Git LFS. Retrieve them after a fresh clone when you need to inspect or
update the media:

```sh
just lfs-pull
```

This step requires [Git LFS](https://git-lfs.com/). It is not required for ordinary library, test,
or documentation changes.

## Development checks

Run the standard local checks with:

```sh
just check-all
```

This matches the formatting, lint, test, and documentation commands used by CI:

```sh
cargo +nightly fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-features
RUSTDOCFLAGS="--cfg docsrs -D warnings" \
  cargo +nightly doc --locked --no-deps --all-features
```

CI also verifies the distributable crate archive from a clean Git checkout:

```sh
cargo package --locked
```

The equivalent shortcut is `just package`. Cargo sees every file in an additional jj workspace as
dirty because the workspace is not a Git worktree. From a jj workspace, verify the same archive
contents while allowing that expected state:

```sh
cargo package --locked --allow-dirty
```

Run the checks relevant to the files you changed. Run `just check-all` and the appropriate package
command before handing off a change intended for integration.

For Markdown changes, lint the repository documentation with its checked-in configuration:

```sh
markdownlint-cli2 "**/*.md"
```

## Running examples

The combined demonstration is the default `just` run target:

```sh
just run
```

Each maintained example can also be run directly:

```sh
cargo run --example linear_spinner
cargo run --example bar_spinner
cargo run --example flux_spinner
cargo run --example circle_spinner
cargo run --example square_spinner
cargo run --example spinner
cargo run --example table_embed
```

The compact gallery application selects a scene after `--`. List its scenes or preview one with:

```sh
cargo run --example readme -- --help
cargo run --example readme -- overview
```

Press `q` or `Esc` to leave an interactive example.

## Contribution expectations

- Keep changes focused and use Conventional Commit descriptions.
- Preserve observable rendering and animation behavior unless the change intentionally updates it.
  Relevant behavior includes direction, endpoint dwell, clipping, styles, alignment, optional
  blocks, and text conversions.
- Test rendering and animation through observable buffers or terminal output. Include boundary
  cases such as zero-sized areas, small containers, oversized geometry, and clipping when they are
  relevant.
- Keep rendering through `Widget for &T` immutable and free of clock reads. Callers provide the
  tick used to select an animation frame.
- Keep terminal backends in development dependencies so they do not expand the library dependency
  surface.
- Keep public documentation examples compilable. Preserve useful documentation when reorganizing
  it, and verify claims against the implementation.
- Preserve upstream authorship, MIT notices, and historical changelog entries.

The repository-specific implementation guidance in [`AGENTS.md`](AGENTS.md) provides more detail
for maintainers and coding agents.

## Visual changes and gallery recordings

When a change affects rendered output, update the corresponding example and recording. Keep
recordings content-sized, hide compilation from the capture, and inspect a still frame for clipping,
alignment, framing, and excess whitespace.

Recording requires [VHS](https://github.com/charmbracelet/vhs). The canonical recording procedure,
other dependencies, scene layout, optimization behavior, and Git LFS handling live in
[`examples/readme/README.md`](examples/readme/README.md). Follow that guide to regenerate all
galleries or preview an individual scene. The `justfile` also exposes `vhs-list` and `vhs-tape` for
listing or recording individual tapes:

```sh
just vhs-tape linear-horizontal
```

## Package and changelog previews

Verify the crate archive without publishing it from a clean Git checkout:

```sh
just package
```

Use `cargo package --locked --allow-dirty` in an additional jj workspace, as described in
[Development checks](#development-checks).

From a conventional Git checkout, preview the unreleased changelog generated by git-cliff without
changing `CHANGELOG.md`:

```sh
just changelog-preview
```

Additional jj workspaces do not provide the resolvable Git `HEAD` that git-cliff requires, so run
the preview from a Git checkout rather than adding or changing Git refs for the workspace.

Publication is currently disabled with `publish = false` while the `tui-spinner` fork is prepared
as `ratatui-spinner`. Do not create tags, publish crates, or enable the disabled release workflows
as part of an unrelated contribution.
