# SlideCraft roadmap

SlideCraft aims at full PowerPoint parity — and to be better: faster, open (a documented zip+JSON
format plus PPTX), scriptable by agents (CLI, JSON control channel, MCP), and available everywhere
(macOS, Windows, Linux, BSD and the web).

## Status (2026-10-07)

**Working today**
- Document model: presentations, slides, masters and 11 standard layouts with placeholder inheritance
  (slide → layout → master → theme), sections, notes, comments, custom shows, header/footer fields,
  embedded media, unlimited undo built on shared slide snapshots.
- Themes: 8 original themes with colour and font schemes, custom colours/fonts, backgrounds,
  slide size presets.
- Shapes: ~150 preset geometries with adjustment handles, text boxes, pictures (with crop and
  adjustments), tables with styles, charts (column, bar, line, area, pie, doughnut, scatter and more),
  basic SmartArt, groups, connectors, ink, action buttons, WordArt.
- Text: in-place editing with caret, selection, keyboard and mouse; every Home-tab font and paragraph
  control; bullets and numbering; levels; autofit; columns; vertical text; fields; hyperlinks.
- Editing: select, marquee, move, resize, rotate, adjust; smart guides; nudge; duplicate; z-order;
  group/ungroup; align/distribute; Format Painter; Selection pane; clipboard incl. images.
- Transitions (fade, push, wipe, split, cover, uncover, zoom, morph…) and animations (entrance,
  emphasis, exit, motion paths, triggers, by-paragraph builds) on a shared timeline engine.
- Slide show: full screen, keyboard/mouse navigation, blank screens, go-to-slide, pen, presenter
  window, rehearse timings, custom shows, Set Up Show; Reading View.
- Views: Normal (thumbnails with sections, slide, notes), Outline, Slide Sorter, Notes Page,
  Slide Master; zoom; grayscale.
- Review: comments, accessibility checker, spelling.
- PDF export: slides, notes pages and handouts (1–9 per page) with a selectable real-text layer, hyperlinks and slide bookmarks (`file.export {format: "pdf", layout}`, File › Export…).
- UI: PowerPoint-style ribbon with contextual tabs, ~240 original icons, status bar, panes, command
  palette, light/dark.
- Automation: ~200 commands, every one reachable from `slidecraft-cli`, the app's JSON control
  channel and the MCP server (headless or connected to the running app).
- Web: `apps/slidecraft-web` runs the same UI in the browser (trunk; WebGPU with WebGL2 fallback),
  opening the sample deck; Open/Insert use the browser file picker, Save/Export download.
- Release CI: pushes to `release` build a draft GitHub Release (macOS universal dmg, Windows
  x64/x86 msi + zip, Linux AppImage/deb/rpm/tar.gz + Flatpak, FreeBSD tar.gz, web zip); signing
  secrets live in the `release` environment. Version: `cargo xtask version`.

**In progress:** PPTX import/export · first signed release run.

**Next (in order):** vector PDF artwork · audio/video playback · Format Shape pane depth · connectors with
glue, freeform and edit points · merge shapes · Animation Pane polish · native macOS menu bar ·
print · Notes/Handout masters · equations · SVG pictures.

Milestone details live in `plan/execution-plan.md` (M0–M14, local planning notes).

## How far from full parity (estimate, 2026-10-07)

**Breadth: ~74% weighted** (P0 core 88%, P1 67%, P2 33%) over the 187 features of the PowerPoint
catalogue, scored row by row in [docs/parity.md](docs/parity.md) (`cargo xtask parity` recomputes
it). Many features scored done still lack some of PowerPoint's options, dialogs or pixel fidelity, so
**overall parity including depth is about 55%**.

**Remaining work: about 230 wall-clock hours of a single Claude Opus 5.5 agent** (±30%), or roughly
80–110 hours with four agents in parallel on separate crates:

| Work | Estimate |
|---|---|
| Open P0 (16 partial, 1 missing: PPTX fidelity, vector PDF, media playback, Format Shape pane, presenter view…) | 30 h |
| Open P1 (23 partial, 10 missing: connectors/glue, freeform, merge shapes, print, multi-monitor, SVG…) | 45 h |
| Open P2 (11 partial, 28 missing: equations, video export, 3-D, remove background, thesaurus…) | 40 h |
| Depth and pixel fidelity of every ribbon group, dialog and pane against PowerPoint | 75 h |
| Performance (incremental rendering, GPU raster) and PPTX corpus hardening | 30 h |
| Platform verification and release hardening (signing, Flatpak, BSD, web) | 10 h |

Basis: the M0 arc (model, renderer, text engine, engine with ~200 commands, ribbon UI, show engine,
CLI and MCP) took roughly 25 wall-clock agent hours; open catalogue rows average 1–2 hours each.

## Agents: CLI and MCP

Every command is reachable from `slidecraft-cli` (`run`, `describe`, `commands`, `app` for the running
window, `render`, `convert`), from MCP (`slidecraft-cli mcp`, optionally `--connect PORT`), and from the
app's JSON control channel (`slidecraft --control PORT`).
