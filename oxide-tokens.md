# Oxide — Color & Token Reference
**oxide-tokens.md · Version 0.9.4 · Moonlight theme · April 2026**

This file is the single source of truth for every color, spacing constant, and design token used in Oxide. All values here are referenced by name throughout `oxide-design-doc.md`. When a value changes, update it here first.

---

## 1. Palette — raw hex values

The Moonlight palette is built on deep indigo-navy foundations. Warm tones (orange, coral, gold) appear only as syntax highlights, semantic status colours, and ephemeral indicators — never in structural chrome.

### 1.1 Surface ramp

Used for all backgrounds, panels, and structural fills.

| Token name | Hex | Lightness | Primary use |
|---|---|---|---|
| `surface/deep` | `#171924` | Darkest | Title bar, terminal viewport, status bar, tab bar background |
| `surface/base` | `#1B1D2B` | — | Window root, deepest background, primary button text |
| `surface/1` | `#1E2030` | — | Sidebar, activity bar, agent panel background |
| `surface/2` | `#222436` | — | Editor canvas, main panel fills |
| `surface/3` | `#2F334D` | — | Selected rows, hover fills, active line highlight background |
| `surface/4` | `#383D5C` | Lightest | Elevated popovers, dropdowns, tooltips |
| `surface/gutter` | `#1F2235` | — | Line number gutter strip, breadcrumb bar |
| `surface/highlight` | `#2A2E47` | — | Active line full-width highlight in editor canvas |
| `surface/border` | `#232740` | — | All 0.5px dividers and panel edges |

### 1.2 Accent ramp — Moon tones

| Token name | Hex | Primary use |
|---|---|---|
| `accent/moon` | `#C099FF` | Active tab indicator, cursor, selection handles, active icon borders, primary button background, agent Code Agent dot, agent status panel badge |
| `accent/sky` | `#82AAFF` | Function names in syntax, folder labels in sidebar, JSX/Svelte tag names, Next.js server boundary icon, file paths in terminal |
| `accent/teal` | `#4FD6BE` | Success states, compiler OK badge, Godot node tint, HMR connected dot, active agent count badge, terminal `✓` lines, Groq latency badge |

### 1.3 Text / Ink ramp

| Token name | Hex | Contrast on `surface/2` | Primary use |
|---|---|---|---|
| `text/primary` | `#C8D3F5` | 9.8:1 | All body text, code default, active filenames, agent message text |
| `text/secondary` | `#7A88CF` | 5.2:1 | Inactive labels, descriptions, property names in inspector, breadcrumb non-final segments |
| `text/muted` | `#3D4468` | 3.1:1 | Line numbers, section headers (chrome only), inactive tabs, close buttons |
| `text/ghost` | `#2E3460` | 2.4:1 | Menu bar items, separator pipes, terminal prompt punctuation, Groq attribution |

> **Note:** `text/muted` and `text/ghost` fall below 4.5:1. They are used only for decorative or redundant chrome elements where a nearby higher-contrast element conveys the same information (e.g. a line number beside the code it labels). Never use them for primary labels.

---

## 2. Syntax token colours

These colours apply identically across all supported languages: TypeScript, TSX, Svelte, GDScript, GLSL, and GDShader. Consistency across language modes is intentional — the cognitive load of switching files is lower when syntax tokens share the same meaning regardless of language.

| Token name | Hex | Scope |
|---|---|---|
| `syn/keyword` | `#C099FF` | `export`, `let`, `const`, `pub`, `func`, `class`, `if`, `return`, `extends`, `import`, `from`, `async`, `await`, `mut` |
| `syn/type` | `#FF98A4` | Type annotations, interface names, class names, struct names, GDScript class names, `Self`, `self`, primitives (`usize`, `bool`) |
| `syn/function` | `#82AAFF` | Function and method identifiers, Svelte lifecycle hooks (`onMount`, `onDestroy`), Godot built-ins (`_ready`, `_process`) |
| `syn/string` | `#C3E88D` | String literals, template literals `` ` ` ``, GDString, raw strings |
| `syn/number` | `#FF966C` | Integer literals, float literals, hex literals, binary literals |
| `syn/macro` | `#FCCB67` | Svelte block tags (`{#if}`, `{#each}`, `{/each}`), Next.js directives (`"use client"`, `"use server"`), GDScript annotations (`@export`, `@onready`, `@tool`), Rust macros (`println!`, `derive`) |
| `syn/comment` | `#4A5172` | Single-line (`//`, `#`), block (`/* */`), HTML/Svelte (`<!-- -->`), doc comments (`///`) |
| `syn/tag` | `#FF98A4` | HTML element names, JSX/TSX component names, Svelte component names, Godot node type names in `.tscn` |
| `syn/attr` | `#82AAFF` | HTML attribute names, JSX prop names, Svelte directives (`on:click`, `bind:value`, `class:`), GDScript property paths |
| `syn/variable` | `#C8D3F5` | Local variable identifiers — uses `text/primary` intentionally (no special tint) |
| `syn/property` | `#B4C2F0` | Object property access (`.name`, `.length`), node property paths, struct field access |
| `syn/shader` | `#4FD6BE` | GLSL built-in functions and types (`vec3`, `uniform`, `varying`, `gl_Position`), GDShader keywords |
| `syn/lifetime` | `#B4F9F8` | Rust lifetime annotations (`'a`, `'static`, `'lifetime`) |
| `syn/operator` | `#89DDFF` | Operators (`=`, `=>`, `&&`, `||`, `?`, `::`, `->`) |
| `syn/punctuation` | `#7A88CF` | Brackets, braces, semicolons, commas (subtle — should not compete with keywords) |

---

## 3. Semantic / status colours

These tokens encode meaning in UI elements: diagnostic severity, build state, HMR state.

| Token name | Hex | Background pairing | Border pairing | Use |
|---|---|---|---|---|
| `status/error` | `#FF98A4` | `#2A1D30` | `#3E2540` | TS errors, GDScript parse errors, red squiggles, error badge text, terminal stderr |
| `status/warning` | `#FCCB67` | `#2A2218` | `#3E3018` | ESLint warnings, unused vars, yellow squiggles, warning badge text |
| `status/info` | `#82AAFF` | `#162040` | `#1D2F48` | Informational diagnostics, hints |
| `status/ok` | `#4FD6BE` | `#162030` | `#1D3A48` | Compiler success, test pass, HMR connected, Vite startup confirmed |

### Badge colour recipes

Each status colour has a corresponding badge recipe. All badge border-radius: 2px, font: monospace 10px.

| Variant | Text | Background | Border |
|---|---|---|---|
| Error | `#FF98A4` | `#2A1D30` | `0.5px #3E2540` |
| Warning | `#FCCB67` | `#2A2218` | `0.5px #3E3018` |
| Info | `#82AAFF` | `#162040` | `0.5px #1D2F48` |
| OK | `#4FD6BE` | `#162030` | `0.5px #1D3A48` |
| Version | `#C099FF` | `#231C3A` | `0.5px #3D2F5E` |
| Channel (stable) | `#4FD6BE` | `#162030` | `0.5px #1D3A48` |
| Channel (nightly) | `#FCCB67` | `#2A2218` | `0.5px #3E3018` |
| Agent active | `#4FD6BE` | `#162030` | `0.5px #1D3A48` |
| Groq latency | `#4FD6BE` | `#162030` | `0.5px #1D3A48` |

---

## 4. File type dot colours

Used in the sidebar filename dot indicator and as the active-tab top border tint when a file-type-specific colour is warranted.

| Extension(s) | Hex | Colour name |
|---|---|---|
| `.svelte` | `#FF98A4` | Svelte rose |
| `.tsx`, `.jsx` | `#82AAFF` | React/Next sky |
| `.ts`, `.js` | `#FCCB67` | TypeScript amber |
| `.gd` | `#C3E88D` | GDScript sage |
| `.tscn` | `#4FD6BE` | Godot scene teal |
| `.tres` | `#B4C2F0` | Godot resource lilac |
| `.glsl`, `.gdshader` | `#FF966C` | Shader orange |
| `.json`, `.env` | `#82AAFF` | Config sky |
| `.md` | `#7A88CF` | Docs secondary |
| `.toml`, `.yaml`, `.yml` | `#FCCB67` | Config amber |
| `.rs` | `#C099FF` | Rust moon |
| `.css`, `.scss` | `#FF98A4` | Style rose |
| `.html` | `#FF966C` | HTML orange |
| Directory | `#82AAFF` | Folder sky |
| Unknown / binary | `#3D4468` | Muted gray |

---

## 5. Terminal ANSI palette

The 16-colour ANSI palette used by the integrated terminal. These values override the system terminal colours when Oxide's terminal is active.

| ANSI index | Role | Hex |
|---|---|---|
| 0 | Black (normal) | `#1B1D2B` |
| 1 | Red | `#FF98A4` |
| 2 | Green | `#C3E88D` |
| 3 | Yellow | `#FCCB67` |
| 4 | Blue | `#82AAFF` |
| 5 | Magenta | `#C099FF` |
| 6 | Cyan | `#4FD6BE` |
| 7 | White | `#C8D3F5` |
| 8 | Bright Black | `#3D4468` |
| 9 | Bright Red | `#FF5370` |
| 10 | Bright Green | `#91B859` |
| 11 | Bright Yellow | `#FFD580` |
| 12 | Bright Blue | `#82AAFF` |
| 13 | Bright Magenta | `#C792EA` |
| 14 | Bright Cyan | `#89DDFF` |
| 15 | Bright White | `#FFFFFF` |
| — | Terminal background | `#171924` (`surface/deep`) |
| — | Terminal foreground | `#C8D3F5` (`text/primary`) |
| — | Selection background | `#2F334D` (`surface/3`) |
| — | Cursor | `#C099FF` (`accent/moon`) |

---

## 6. Agent panel colours

Colours specific to the AI Agents panel and Groq integration.

| Token name | Hex | Use |
|---|---|---|
| `agent/panel-bg` | `#1B1D2B` | Agent panel background (matches `surface/base`) |
| `agent/user-bubble-bg` | `#231C3A` | User message bubble background |
| `agent/user-bubble-border` | `#3D2F5E` | User message bubble border |
| `agent/bot-bubble-bg` | `#1E2030` | Agent message bubble background (matches `surface/1`) |
| `agent/bot-bubble-border` | `#2F334D` | Agent message bubble border (matches `surface/3`) |
| `agent/code-bg` | `#171924` | Code block inside agent message (matches `surface/deep`) |
| `agent/code-border` | `#232740` | Code block border (matches `surface/border`) |
| `agent/code-agent-dot` | `#4FD6BE` | Code Agent status dot (matches `accent/teal`) |
| `agent/route-agent-dot` | `#C099FF` | Route Agent status dot (matches `accent/moon`) |
| `agent/writing-dot` | agent-role colour | Writing indicator dot — inherits the agent's dot colour |
| `agent/chip-bg` | `#1E2030` | Quick-action chip background |
| `agent/chip-border` | `#232740` | Quick-action chip border |
| `agent/chip-text` | `#7A88CF` | Quick-action chip label |
| `agent/groq-attribution` | `#2E3460` | "Powered by Groq" line text (`text/ghost`) |

---

## 7. Spacing & radius constants

| Token name | Value | Use |
|---|---|---|
| `space/1` | 4px | Minimum gap, icon-to-label |
| `space/2` | 8px | Component internal gaps |
| `space/3` | 12px | Tab padding, code left padding |
| `space/4` | 14px | Panel internal padding (x) |
| `space/5` | 16px | Scene tree indent, section gaps |
| `space/6` | 20px | Card padding |
| `radius/sm` | 2px | Badges, pills |
| `radius/md` | 3px | Buttons, tabs |
| `radius/lg` | 4px | Code blocks, inputs |
| `radius/xl` | 6px | Message bubbles |
| `radius/card` | 10px | Editor window chrome |

---

## 8. Typography constants

| Token name | Value |
|---|---|
| `font/mono` | `'SF Mono', 'Cascadia Code', 'Fira Code', 'Courier New', monospace` |
| `font/sans` | System sans (`-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif`) |
| `font/editor-size` | 12px |
| `font/terminal-size` | 11px |
| `font/ui-size` | 11px |
| `font/agent-size` | 11.5px |
| `font/badge-size` | 10px |
| `font/label-size` | 9px |
| `font/editor-line-height` | 1.65 |
| `font/terminal-line-height` | 1.75 |
| `font/agent-line-height` | 1.55 |
| `font/min-size` | 11px (9px for uppercase chrome labels only) |

---

## 9. Motion constants

| Token name | Value | Use |
|---|---|---|
| `motion/instant` | 0ms | Tab switches, tree expand/collapse, terminal resize |
| `motion/fast` | 80ms ease | Hover background transitions |
| `motion/badge` | 120ms ease-out | Badge opacity on appear |
| `motion/panel` | 120ms ease-out | Agent panel collapse/expand (width) |
| `motion/tooltip` | 100ms ease | Tooltip opacity on appear |
| `motion/hmr-flash` | 200ms ease-out | HMR reload canvas flash |
| `motion/notification` | 160ms ease-out | Notification slide-in (transform from bottom) |
| `motion/blink-period` | 0.9s step-end | Cursor and agent writing-indicator blink |

All transitions must be wrapped in:
```css
@media (prefers-reduced-motion: no-preference) {
  /* transition rules here */
}
```
Fallback for reduced-motion: instant (0ms).

---

## 10. Border constants

| Token name | Value | Use |
|---|---|---|
| `border/default` | `0.5px solid #232740` | All structural dividers, panel edges, card outlines |
| `border/emphasis` | `0.5px solid #2F334D` | Hover-state borders, input focus adjacent |
| `border/focus` | `2px solid #C099FF` | Keyboard focus rings (offset 2px) |
| `border/active-tab` | `1.5px solid #C099FF` | Active tab top indicator — only 1.5px exception |
| `border/user-bubble` | `0.5px solid #3D2F5E` | User message bubbles in agent panel |

---

*oxide-tokens.md — single source of truth for all Oxide design tokens. Do not hardcode hex values anywhere in the codebase; always reference tokens by name.*
