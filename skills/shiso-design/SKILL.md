---
name: shiso-design
description: Use when building or changing any Shiso user interface (web, plain HTML pages, React Native, egui/Tauri desktop). Covers the shared tokens, which component to use, and the rules that keep every Shiso surface looking the same.
---

# Shiso design system

Source: https://github.com/sercand/shiso-design. One token file feeds every platform.
Never hard-code a color, font, radius or control height; if a value is missing, add a token.

## The look

- Accent: Aojiso green (`primary`). Greys are cool (blue-tinted). Light and dark are both first-class.
- Type: Hanken Grotesk for everything, JetBrains Mono for ids, code and keys. Weights 400/500/600/700.
- Shape: soft. Controls 10px radius, cards 16px, pills fully round.
- Surfaces: lifted. White cards with a soft shadow on a tinted canvas; inputs are filled, with no border at rest.
- Density: compact on web and desktop (30px controls, 13px text); comfortable on touch (40px, 15px).

## Where things are

| Need | Use |
| --- | --- |
| Change a color, size, radius | `tokens/tokens.json`, then `npm run build`. Generated files are never edited by hand. |
| Plain HTML page, no build step | `packages/css/shiso.css` (tokens + components) and `packages/css/fonts.css` |
| Tailwind / shadcn app | `packages/css/tokens.css` + `packages/tailwind/theme.css` (maps tokens to the shadcn variable names) |
| React Native | `packages/native` (`ShisoProvider`, `useShiso`, `Text`, `Button`, `Card`, `Pill`, `Input`, ...) |
| Rust | `crates/shiso-theme` (`tokens`), feature `egui` for `shiso_theme::egui::install` and widgets |
| See it running | `apps/html`, `apps/native`, `apps/egui`. All three show the same screen; keep them in step. |

## Color roles

| Token | For |
| --- | --- |
| `canvas` | The page behind everything, including the sidebar |
| `card` | Lifted surfaces: cards, popovers, the selected nav item, the selected segment |
| `fill` / `fill-hover` | Inputs, secondary buttons, insets inside a card, neutral pills |
| `line` | Hairlines: table rows, dividers, outline buttons |
| `text` / `text-2` | Primary and secondary text |
| `primary` / `primary-hover` / `primary-text` | The main action, checked controls, meters, focus ring |
| `soft` / `soft-text` | A tinted, low-emphasis accent: avatars, in-progress pills |
| `live` | Only for "an agent is working right now" |
| `ok`, `warn`, `bad` (+ `-bg`) | Status. Never used as decoration, never as the accent |

## Rules

1. One primary button per view. Everything else is secondary, outline or ghost.
2. Do not nest cards. Group inside a card with an inset (`.s-inset`, `Inset`, `shiso::inset`).
3. Status is a pill with a dot and a word; color alone never carries meaning.
4. Numbers that line up use tabular figures (`.s-num`, `fontVariant: ["tabular-nums"]`). Money keeps its currency symbol.
5. Ids and keys (`OPS-142`, `cr_8f3a2c`) are mono and `text-2`.
6. No borders on cards or inputs in light mode; separation comes from the canvas, fill and shadow. Dark mode swaps the shadow for a 1px highlight, which the tokens already do.
7. Sentence case everywhere. Buttons say what happens ("Approve", "Save changes"), never "OK" or "Submit".
8. Every screen must read correctly in light and dark. Check both before finishing.
9. Touch targets are at least 44px on mobile even when the control is drawn smaller.
10. Icons are Lucide, 1.75px stroke, sized to the text (about 1.15em).

## Per platform

**HTML.** Put `class="shiso"` on `<body>`. Theme follows the system; pin it with `data-theme="light|dark"` on `<html>` (the `.dark` class also works). Density with `data-density="compact|comfortable"`. Classes are `s-*`: `s-btn s-btn--primary`, `s-input`, `s-card`, `s-pill s-pill--warn s-pill--dot`, `s-seg`, `s-table`, `s-shell`, `s-chip` (filter toggle), `s-alert s-alert--bad`, `s-code`, `s-facts`, `s-topbar`, `s-btn--sm`. A card that is a link is `<a class="s-card">`.

**Tailwind / shadcn.** Import the tokens and the theme; existing shadcn classes (`bg-background`, `text-muted-foreground`, `bg-primary`) then resolve to Shiso tokens. Extra roles are `bg-shiso-soft`, `text-shiso-live`, and so on.

**React Native.** Wrap the app in `<ShisoProvider>`, load `fontAssets` with `expo-font`, and use the package's `Text` instead of React Native's so fonts apply. Read raw values with `useShiso()`.

**egui.** Call `shiso_theme::egui::install(&ctx, tokens::COMPACT)` once. Stock widgets are themed; use `shiso::card`, `shiso::inset`, `shiso::primary_button`, `shiso::pill`, `shiso::meter` and `shiso::text::{h1,h2,h3}` for the rest. egui has no font weights, so weights are families: `shiso::medium()`, `semibold()`, `bold()`.

## Checking your work

- `npm run build` regenerates tokens for all platforms.
- `cargo run -p shiso-egui-demo`, or `SHISO_SCREENSHOT=out.png SHISO_THEME=dark cargo run -p shiso-egui-demo` to get a PNG without opening the window for long.
- `cd apps/native && npm run typecheck && npx expo start`.
- Open `apps/html/index.html` through any static server started at the repo root.
