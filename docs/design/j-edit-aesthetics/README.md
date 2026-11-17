# J-Edit aesthetic pass — amp/cabinet art (staging)

Reference material for adding J-Edit-inspired graphics to the amp/cabinet
pickers. Direction settled on: a real-photo macro crop (knob row for amps,
grille-cloth + corner for cabinets/acoustics), in the style of Kemper/Big
Hairy Profiles pack art, so users can recognize gear while framing out
logos/trademarks instead of retouching them out.

All sources are CC-licensed or public-domain photos from Wikimedia Commons;
see `MANIFEST.md` for per-image attribution, license, and source URL for
every entry, plus notes on substitutions, weak crops, and items flagged for
a human decision before shipping (fuzz pedal stand-in, '78 Marshall
substitution, two low-quality "weak" crops).

`crops/` holds the 24 finished crops out of 44 catalog entries (amp models
in `src/jstation/data/dsp/amp.rs`, cabinet types in
`src/jstation/data/dsp/cabinet.rs`); the rest had no usable Commons source
or are intentional skips (bypass/no-cabinet modes).

**Status: not yet wired into the app.** Nothing here is loaded by the UI
yet — this is staged source material pending a decision on the remaining
gaps and how the images get embedded/rendered in `src/ui`.
