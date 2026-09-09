# README example

This folder contains the `readme` example that produces the compact, caption-free mini-galleries.
`main.rs` owns the CLI and animation loop, and each scene has a matching source module. The existing
integration demos remain available as additional code examples.

`../vhs/overview.tape` produces the curated hero image for the README, crates.io, and showcase
submissions. It shows one compact, representative view of every spinner family; the remaining
tapes in `examples/vhs/` provide detailed configuration galleries.

Build the example once, then regenerate every gallery item:

```sh
cargo build --release --example readme
just vhs-all
```

`just vhs-all` losslessly optimizes the generated GIFs with ImageMagick. Run `just vhs-optimize` to
repeat that optimization without recording the tapes again. The optimizer keeps the original VHS
output whenever its candidate would be larger. This reduces the animation-heavy recordings
substantially: the current Flux gallery drops from 32.4 MB to 5.0 MB, the Bar galleries from roughly
9–10 MB to about 1 MB, and the Linear galleries from 2.4–5.3 MB to 0.2–0.6 MB.

The recipes finish by passing each GIF through the Git LFS clean filter. This is necessary because
jj does not apply that filter when it snapshots the working copy. `just vhs-optimize`
materializes the GIF data temporarily when the checked-out files already contain LFS pointers.

To preview a scene interactively, run `cargo run --example readme -- <scene>` and press `q` to quit.
Use `--help` in place of a scene name to list all scenes.

The recipe runs VHS with `NO_COLOR` unset. The tapes use the default font, 30 pixels of padding, and
explicit pixel dimensions. Each tape writes a representative PNG frame to `target/vhs/`. GIFs
are written to `examples/vhs/generated/` and are tracked by the repository's existing `*.gif` Git
LFS rule.

Linear and Bar use separate horizontal and vertical tapes so each direction remains compact and
readable. Flux uses one gallery for every frame preset plus its direction, phase, size, and
custom-frame settings. Circle, Square, and Rect each use one gallery for their geometry, direction,
and other shape-specific settings. `../vhs/text.tape` shows `Into<Text>`, `to_text()`, and
`to_lines()` conversions in table cells.
