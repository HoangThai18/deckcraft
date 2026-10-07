# Attribution

Every asset SlideCraft includes, bundles, embeds at build time or shows in its documentation, with
its author, source and licence. The policy is in [AGENTS.md](AGENTS.md) §1. `cargo xtask assets`
fails if a file under `assets/`, `docs/images/` or `docs/brand/` is missing from this table.

SlideCraft contains **no Microsoft, Adobe, Avid or Autodesk iconography, images, fonts, themes,
templates or other assets**. Its themes, colour schemes, layouts, preset-shape geometry, sample
deck and UI icons are original work of the SlideCraft contributors (drawn or generated in code).

## In this repository

| Asset | Author | Source | Licence | Used for |
|---|---|---|---|---|
| `docs/brand/artcraft-logo.svg`, `docs/brand/artcraft-logo.png` | ArtCraft Team | ArtCraft brand kit (project owner) | ArtCraft trademark, `docs/brand/LICENSE-brand.txt` (not open source) | README header |
| `docs/brand/artcraft-logo-white.svg`, `docs/brand/artcraft-logo-white.png` | ArtCraft Team | ArtCraft brand kit | ArtCraft trademark, `docs/brand/LICENSE-brand.txt` | README header (dark mode) |
| `docs/brand/artcraft-mark.svg`, `docs/brand/artcraft-mark.png` | ArtCraft Team | ArtCraft brand kit | ArtCraft trademark, `docs/brand/LICENSE-brand.txt` | README footer, About |
| `docs/brand/artcraft-mark-black.svg`, `docs/brand/artcraft-mark-black.png` | ArtCraft Team | ArtCraft brand kit | ArtCraft trademark, `docs/brand/LICENSE-brand.txt` | Mark for light backgrounds |
| `docs/brand/LICENSE-brand.txt` | ArtCraft Team | craftrules `standards/license-files` | — (licence text) | Terms for the ArtCraft marks |

## Bundled through dependencies

| Asset | Author | Source | Licence | Used for |
|---|---|---|---|---|
| Ubuntu Light (`Ubuntu-Light.ttf`, via the `epaint_default_fonts` crate 0.36) | Dalton Maag Ltd for Canonical | https://crates.io/crates/epaint_default_fonts | Ubuntu Font Licence 1.0 | Last-resort text face when no other font is available; UI fallback |
| Hack Regular, Noto Emoji, emoji-icon-font (via `epaint_default_fonts` / egui) | Source Foundry; Google; Jakub Steiner et al. | https://crates.io/crates/epaint_default_fonts | MIT / Bitstream-Vera, OFL-1.1, MIT | egui UI fallback glyphs |

## Embedded at build time from craft-fonts (`CRAFT_FONTS_DIR`)

Release builds (and local builds with a craft-fonts checkout) embed the fonts listed in
[craft-fonts](https://github.com/storytold/craft-fonts) `fonts/latin-manifest.txt` and
`fonts/manifest.txt` (per-file authors, sources, SHA-256 and licences in its
[ATTRIBUTION.md](https://github.com/storytold/craft-fonts/blob/main/ATTRIBUTION.md)), except the
families excluded by `crates/fonts/build.rs`:

| Families | Licence | Notes |
|---|---|---|
| Inter | OFL-1.1 | Default theme font |
| Carlito, Caladea | OFL-1.1 | Metric-compatible stand-ins for documents that ask for Calibri / Cambria |
| Liberation Sans, Serif, Mono | OFL-1.1 | Metric-compatible stand-ins for Arial, Times New Roman, Courier New |
| Montserrat, Lato, Open Sans, Roboto, Merriweather, Playfair Display, Poppins, Nunito Sans | OFL-1.1 | Theme fonts |
| BIZ UDPGothic, BIZ UDMincho, Shippori Mincho, Noto Sans Arabic | OFL-1.1 | Japanese and Arabic text |

Web (wasm32) builds embed only Inter Regular/Bold and BIZ UDPGothic Regular.
