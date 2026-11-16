# Theme contract

Dlea's visible theme is resolved from a small set of files and re-resolved live
when any of them changes. This document is the contract a shell or theme
pipeline (for example nosDshell's `Assets/Templates/dlea-*.toml`) writes
against. Everything here is additive to the packaged theme: an empty
`~/.config/dlea` means Dlea behaves exactly as it does on a stock Omarchy
install.

## Input files and precedence

| File | Role |
|---|---|
| `~/.local/state/omarchy/current/theme/colors.toml` | Packaged palette (optional on non-Omarchy systems) |
| `~/.config/dlea/colors.toml` | Palette override — appended key by key, so keys it sets win and keys it omits keep the packaged value |
| `~/.local/state/omarchy/current/theme/shell.toml` | Packaged style values |
| `~/.config/dlea/shell.toml` | Style override — same key-by-key overlay |
| `~/.config/omarchy/shell.toml` | User style layer, applied last |

Both `~/.config/dlea` files are watched, including the directory itself:
creating, replacing or deleting them repaints the window without a restart. On
a system with no Omarchy state directory, `~/.config/dlea/colors.toml` alone is
sufficient — it can supply the whole palette.

The GPUI window reads the same contract through
`crates/dlea-gui/src/presentation/source.rs`; the TUI and shared theme code read
it through `src/theme/watch.rs` (`ThemePaths::load`). Both apply the overlay
identically.

## colors.toml keys

One `key = "#rrggbb"` per line; unquoted and single-quoted values parse too.
Named keys:

| Key | Role |
|---|---|
| `background` | Window ground |
| `foreground` | Primary text |
| `accent` | Accent: cursor wash, checked state, links |
| `dark_background` | Chrome plane — rail, toolbar, status bar (the `surface` role) |
| `bright_foreground` | Emphasis text |
| `muted` | Secondary text; its darker derivative frames it |
| `dark_foreground` | Deepest dim ink; also the Gray tag swatch |
| `red` | Urgent/error ink and the Red tag (the last `red`/`color1` line wins) |
| `orange`, `yellow`, `green`, `blue`, `magenta` | Tag swatches |
| `cyan` | Symlink ink (and the Cyan-adjacent tag) |
| `green` | Executable ink and the Green tag |
| `color0`–`color15` | Base16 fallbacks for `background`, `foreground`, `accent`, `bright_foreground`, tag and `red` when the named keys are absent |

Colors are lifted to WCAG contrast where the role requires it, so a theme only
names hues, not accessibility variants.

## shell.toml sections

`[section] key = value`. Both `[style]` and `[controls]` feed the same control
table; the keys below are written as `controls.*` values.

`[font]`: `base-size` (default 12), `body`, `body-small`, `caption` — pixel
sizes the GPUI window's font ladder reads.

`[dlea]`: `background-alpha` (0.0–1.0) — pins the glass background alpha when a
compositor toggle asks for it.

`[spacing]`: `scale`, `scale-with-font`, and per-name pixel overrides
(`control-height`, `popup-row-height`, `row-gap`, `row-padding-x`,
`panel-padding`, `popup-padding`, `control-gap`, `control-padding-x`,
`control-padding-y`, `input-padding-y`, `dropdown-width`,
`searchable-dropdown-width`, `number-field-width`,
`searchable-popup-min-height`, `label-gap`, `panel-gap`, `xxs`–`huge`).

`[controls]` / `[style]` state recipe — for each state `normal`,
`hover-cursor`, `selected`, `pressed`, `focus`, `selection`:
`<state>-color` (a name: `foreground`, `accent`, `urgent`, `background`,
`transparent`, or `#rrggbb`), `<state>-fill-alpha`, `<state>-border-width`,
`<state>-border-alpha`.

Wash roles (single colors, not per-state):

| Key | Default |
|---|---|
| `cursor-wash-color` | `accent` — the cursor row, alpha from `selected-fill-alpha` |
| `menu-cursor-color`, `menu-cursor-fill-alpha` | `accent` @ 0.20 |
| `window-hairline-color`, `window-hairline-alpha` | `foreground` @ 0.10 |
| `rule-color`, `rule-alpha` | muted-frame @ 0.4 |
| `primary-wash-color`, `primary-wash-alpha` | `accent` @ 0.14 |

## The DDE/GXDE recipe

nosDshell's template (`Assets/Templates/dlea-*.toml`) ships the gxde-file-manager
dark-theme recipe on top of the wallpaper-derived palette:

- resting controls tonal `foreground @ 0.15`, no outline;
- hover `foreground @ 0.10` (GXDE `#1Affffff`);
- checked/cursor `accent @ 0.20` with accent ink (GXDE `#332ca7f8` + `#2ca7f8`);
- multi-select plate `accent @ 0.85` (GXDE `#da2ca7f8`);
- hairlines under `foreground @ 0.10`; menus hover accent @ 0.20;
- `font.base-size = 12`.

The toolbar follows `gxde-file-manager`'s `DFileManagerWindow.theme` on top of
that recipe: a joined back/forward/up group, a 4 px crumb well, and a view
tray. Dark plates use foreground at 15% / 30% / 10% (rest / hover / press).
Row and menu washes stay on the bookmark recipe above. With no Omarchy palette
and no `~/.config/dlea/colors.toml`, the window ground is the GXDE dark
`#202020` / accent `#2ca7f8`. What matches, and what does not, is listed in
[gxde-fidelity.md](gxde-fidelity.md).

Set only the keys a template owns; the rest fall through to Dlea's defaults or
the packaged theme.

## Scope

This contract covers colors, the control/state recipe, font sizes, spacing and
the glass alpha. Window decoration (`decoration.rounding`, `gapsOut`,
`reducedMotion`) still comes from the Hyprland inputs `src/theme/watch.rs`
already watches — it is compositor configuration, not theme content.
