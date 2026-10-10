# Repository Guidelines

## Project structure

`ratatui-spinner` provides animated spinner widgets for Ratatui. Public widgets are re-exported
from `src/lib.rs`; their implementations and unit tests live in the corresponding modules.
Interactive examples live in `examples/`, with VHS recordings defined in `examples/vhs/`.

## Development commands

- Format with `cargo +nightly fmt`; check with `cargo +nightly fmt --check`. The configuration
  follows Ratatui formatting conventions and uses nightly comment and import formatting options.
- Run `cargo clippy --locked --all-targets --all-features -- -D warnings`.
- Run `cargo test --locked --all-features` for unit tests and doctests.
- Build documentation with
  `RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc --locked --no-deps --all-features`.
- Run `cargo package --locked` to verify the distributable without uploading it.
- `just check-all` runs formatting, clippy, tests, and documentation checks.

## Code organization

- Keep code in top-to-bottom reading order and prefer local, linear control flow.
- Add a helper when it reduces the reasoning burden or replaces explanatory comments without
  creating a parameter-heavy interface.
- Prefer one fallible operation per line when it makes the failure site easier to identify.
- Use explanatory variables to keep nested Rust expressions shallow and make intermediate intent
  visible.

## Widget behavior and API

- Preserve all six visual families: Linear, Rect, Square, Circle, Bar, and Flux.
- Preserve customization, motion and direction, endpoint dwell, clipping, styles, alignment,
  optional blocks, and text conversions when changing rendering or animation internals.
- Keep rendering through `Widget for &T` immutable and free of clock reads. Owned widget
  implementations may delegate to the reference implementation.
- Prefer clear fluent builders with useful defaults and `#[must_use]` where appropriate.
- Keep internal macros private. Reuse existing style and rendering helpers when they fit;
  do not force geometry-specific behavior through a generic rendering path.
- Scope lint exceptions narrowly and explain the invariant that makes a conversion safe.
- Keep terminal backends in development dependencies rather than the library dependency surface.

## Tests and examples

- Test observable rendering and animation behavior, not just setter field assignments.
- Use deterministic buffers or TestBackend fixtures for terminal output. Cover zero areas,
  small containers, oversized geometry, clipping, and text conversions.
- Keep public documentation examples compilable and examples useful for visual comparison.
- Prefer fixed lengths for UI elements intended to stay compact.
- Retain VHS tapes and write output to `examples/vhs/generated/`. GIFs use Git LFS; use
  `just lfs-pull` to retrieve media for the checked-out revision.
- Hide compilation in recordings and regenerate affected demos when visual behavior changes.

## Documentation and showcase media

- When reorganizing documentation, preserve its informational coverage unless removal is
  intentional. Use the previous document as a checklist and verify technical claims against the
  source rather than carrying them forward unexamined.
- Lead README and showcase material with a compact overview that communicates the library's range
  and useful configuration at a glance. Follow it with focused examples and reference detail.
- Prefer content-sized recordings over terminal-shaped captures. Keep recording dimensions fitted
  to the rendered character grid, and inspect a still frame for clipping, alignment, framing, and
  excess whitespace before accepting the animation.
- Use a stable frame schedule that accounts for rendering time rather than adding a full delay
  after each frame.
- Losslessly optimize generated GIFs, but keep the original when optimization increases its size.
  Keep the measured file-size rationale near the generation workflow.

## Maintenance and provenance

- Preserve upstream authorship, MIT notices, and historical changelog entries.
- Use small focused changes and Conventional Commit descriptions.
- Prefer compatible lockfile updates. Raise dependency minimums only when required by behavior
  or integration, and review incompatible upgrades separately.
- Keep changelog generation available through git-cliff. Do not create tags, publish crates,
  or enable automatic releases as a side effect of dependency maintenance.
- Publication is disabled with `publish = false` while the migration and release process are
  under review.
