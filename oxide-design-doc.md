# Oxide — Design Document
**Version 0.9.4 · Moonlight theme · April 2026**

---

## 1. Product overview

Oxide is a memory-safe, GPU-accelerated code editor written in Rust. It is purpose-built for three ecosystems: **SvelteKit**, **Next.js**, and **Godot**. Rather than being a generic editor with plugin support bolted on, Oxide ships with first-class language intelligence, project scaffolding, scene/component previews, an integrated AI agent system powered by Groq, and a full terminal — all for each of these three targets out of the box.

The interface is designed around one principle: the code is the product — every UI decision reduces friction between thought and keystroke.

### Target personas

| Persona | Primary stack | Key need |
|---|---|---|
| Web app developer | SvelteKit or Next.js | Fast TypeScript/JSX intellisense, hot-reload preview pane |
| Fullstack engineer | Next.js (App Router) | Server/client boundary visualisation, `.env` management |
| Indie game developer | Godot (GDScript / C#) | Scene tree inspector, shader editor, asset browser |
| Crossover builder | SvelteKit + Godot (HTML5 export) | Shared workspace, export pipeline, agent-assisted scaffolding |

---

## 2. Brand

| Property | Value |
|---|---|
| Name | OXIDE |
| Logotype | `Fe` monogram inside a hexagonal badge |
| Tagline | Memory-safe editor for the web and the world |
| Version badge | `v0.9.4` — lilac on deep aubergine |
| Channel badge | `stable` — teal on deep navy |

See `oxide-tokens.md` for all color values.

---

## 3. Layout overview

The Oxide window is composed of six regions stacked and nested:

```
┌─────────────────────────────────────────────────────────┐
│ Title bar (30px)                                        │
├──────┬──────────────┬──────────────────────┬────────────┤
│      │              │  Tab bar (34px)       │            │
│      │              ├──────────────────────│            │
│      │              │  Breadcrumb (24px)   │            │
│  A   │   Sidebar    ├──────────────────────│  AI Agents │
│  c   │   (200px)    │                      │  panel     │
│  t   │              │  Editor canvas       │  (280px)   │
│  i   │              │                      │            │
│  v   │              ├──────────────────────│            │
│  i   │              │  Terminal (168px)    │            │
│  t   │              ├──────────────────────┤            │
│  y   │              │  Status bar (22px)   │            │
│      │              │                      │            │
└──────┴──────────────┴──────────────────────┴────────────┘
```

### Chrome dimensions

| Region | Size |
|---|---|
| Activity bar | 42px wide |
| Sidebar (default) | 200px wide, resizable 140px–400px |
| Tab bar | 34px tall |
| Breadcrumb row | 24px tall |
| Editor canvas | fills remaining vertical space |
| Terminal panel (default) | 168px tall, resizable |
| Status bar | 22px tall |
| AI Agents panel | 280px wide, collapsible |
| Activity icon hit area | 42×38px |

### Spacing units

Base unit: 4px. All padding values are multiples of 4.

| Use | Value |
|---|---|
| Panel internal padding (x) | 14px |
| Code left padding (after gutter) | 12px |
| Line number gutter width | 38px |
| Tab horizontal padding | 12px |
| Badge padding | 2px 7px |
| Scene tree row indent | 16px per level |
| Terminal line padding (x) | 14px |
| Agent message bubble padding | 8px 11px |

### Border weight

All structural borders: `0.5px solid #232740`. The sole exception is the active-tab top indicator: `1.5px solid #C099FF`.

---

## 4. Typography

Oxide uses a monospace-first type stack. No variable-width UI font is used inside the editor canvas — only in surrounding chrome.

### Type scale

| Role | Family | Size | Weight |
|---|---|---|---|
| Editor code | `'SF Mono', 'Cascadia Code', 'Fira Code', monospace` | 12px | 400 |
| Terminal text | Monospace stack | 11px | 400 |
| Active tab label | Monospace stack | 11.5px | 400 |
| Inactive tab label | Monospace stack | 11.5px | 400 (color `text/muted`) |
| Sidebar filename | Monospace stack | 11.5px | 400 |
| Agent message | Monospace stack | 11.5px | 400 |
| Agent code block | Monospace stack | 10.5px | 400 |
| Section header | System sans | 9px | 600 (all-caps, 0.10em spacing) |
| Breadcrumb | Monospace stack | 11px | 400 |
| Status bar | Monospace stack | 11px | 400 |
| Badge / pill | Monospace stack | 10px | 400 |
| Brand logotype | Monospace stack | 18px | 700 (0.16em spacing) |

Line height: `1.65` in the editor canvas. Terminal: `1.75`. Agent messages: `1.55`. All chrome uses explicit height-based layout at `1.0`.

---

## 5. Component specifications

### 5.1 Title bar

Height 30px, background `surface/deep` (`#171924`). Left: traffic-light controls (11px circles — red `#FF5F56`, amber `#FFBD2E`, green `#27C93F`) followed by menu items in `text/ghost`. Centre: filename and project in `text/ghost`. Right: version and channel badges.

### 5.2 Activity bar

Width 42px, background `surface/deep`. Icons are inline SVG 15×15px, outline style, 1px stroke in `text/muted` when inactive.

**Inactive state** — stroke `text/muted`, no background.
**Hover state** — background `surface/3` (`#2F334D`), 80ms ease.
**Active state** — `2px left border accent/moon`, background `#1B1D2B`.

The AI Agents icon (person with `+` circle) uses `accent/moon` stroke and shows a teal dot when at least one agent is active.

| Icon | Description |
|---|---|
| Files | Document rect with folded corner |
| Search | Circle + diagonal line |
| Source control | Three-node triangle graph + count badge |
| AI Agents | Person silhouette + teal status dot |
| Settings | Circle with radiating tick lines |

### 5.3 Sidebar file tree

Root item: 9px uppercase/600 label in `text/muted`. Folders in `accent/sky` (`#82AAFF`). Active file row background `surface/3`. Inactive file dots in `text/ghost`. File type dot colours follow the token table in `oxide-tokens.md` §4.

### 5.4 Tabs

**Active tab** — background `surface/2` (`#222436`), `1.5px` top border `accent/moon`, dot `●` 8px in `accent/moon`, close `×` in `text/muted`.
**Inactive tab** — background `surface/1` (`#1E2030`), transparent top border, all text in `text/muted`.
**Modified tab** — dot indicator stays visible in place of the close button until saved.
**Unsaved file dot colour** — inherits file type tint from the token table.

### 5.5 Breadcrumb

Height 24px, background `#1F2235`. Segments separated by `›` in `surface/border`. Final segment in `text/secondary`. Clicking any segment opens a file picker for that directory level.

### 5.6 Editor canvas

Background `surface/2`. Line numbers in `text/ghost` on a slightly darker `#1F2235` strip. Active line highlight: full-width `#2A2E47` block. Cursor: 2px block in `accent/moon`.

### 5.7 Badges

| Variant | Background | Text | Border |
|---|---|---|---|
| Error | `#2A1D30` | `#FF98A4` | `0.5px #3E2540` |
| Warning | `#2A2218` | `#FCCB67` | `0.5px #3E3018` |
| OK / success | `#162030` | `#4FD6BE` | `0.5px #1D3A48` |
| Version | `#231C3A` | `#C099FF` | `0.5px #3D2F5E` |

Border radius: 2px. Font: monospace 10px.

### 5.8 Buttons

**Primary** — background `accent/moon` (`#C099FF`), text `surface/base` (`#1B1D2B`) weight 700, border-radius 3px, padding 3px 10px.
**Secondary** — transparent background, `0.5px border #2F334D`, text `text/secondary`, border-radius 3px.
**Ghost** — no border, no background, text `text/muted`. Used for icon buttons inside panels.

### 5.9 Status bar

Height 22px, background `surface/deep` (`#171924`). Left: LSP indicator in `accent/moon`, then diagnostic counts in semantic colours. Right: cursor position, encoding, language mode, bundler version. Separator pipes in `#232740`.

---

## 6. Framework-specific panels

### 6.1 SvelteKit panel

**Route tree** — mirrors `src/routes/`. File colours: `+page.svelte` → `#FF98A4`, `+layout.svelte` → `accent/moon`, `+server.ts` → `#FCCB67`, `+error.svelte` → `status/error`. Indented 16px per level.

**Component graph** — live dependency graph. Nodes: `surface/3` fill + `accent/moon` stroke. Edges: 0.5px `text/muted`. Clicking opens the file.

**HMR status dot** — `●` in status bar. Teal (`accent/teal`) connected; rose (`status/error`) disconnected.

**Store inspector** — lists `$store` references with type signatures in `text/secondary`.

### 6.2 Next.js panel

**App Router tree** — server components: faint `surface/3` background. Client components: `1px accent/moon` left border.

**Boundary gutter icons** — 10px shield at `"use client"` / `"use server"` lines. `accent/sky` for server; `accent/moon` for client.

**`.env` manager** — values blurred by default (`filter: blur(4px)`). Alphabetically sorted by prefix. Environment selector dropdown in panel header.

**Build output panel** — horizontal bar chart by route. Teal < 50 kB, amber 50–150 kB, rose > 150 kB.

### 6.3 Godot panel

**Scene tree** — node type icons at 10px: `Node2D` orange diamond, `Control` blue square, `Area2D` teal circle, `Camera2D` amber lens, `RigidBody2D` coral hexagon.

**Inspector** — two-column table. Property names `text/secondary`, values `text/primary`. Editable values open inline inputs. `@export` properties tagged with `◈` in `accent/moon`.

**GDScript diagnostics** — parse errors: `●` `status/error` in gutter. Warnings: `status/warning`. Hover shows Godot error code in `surface/4` tooltip.

**Shader editor** — horizontal split: code left, preview canvas right (UV debug or solid colour swatch). GLSL built-ins in `syn/shader`.

**Asset browser** — grid of `res://` assets. Images as thumbnails on `surface/3`. Audio shows static waveform in `accent/sky`. Other files show filetype badge.

---

## 7. AI Agents panel

The AI Agents panel is a first-class right-side panel powered by the Groq API. It replaces the traditional single-model assistant with a multi-agent workspace where distinct agents can run in parallel, each with its own role, model, and conversation thread.

### 7.1 Panel structure

The panel is 280px wide and divided into four vertical zones:

**Header zone** (top, ~90px) contains: the panel title "AI AGENTS" with the hex logo badge, an active-agent count badge in `accent/teal`, a model selector dropdown listing all available Groq models, and a Chat/Studio/Agent mode toggle (three-segment control).

**System prompt zone** (~66px) contains a resizable textarea for the shared system prompt. Background `surface/1`, border `surface/3`. Text `text/secondary` when unfocused.

**Chat thread zone** (flex, scrollable) contains agent conversation threads. Multiple agents are separated by a `0.5px surface/border` rule. Each agent section begins with an agent header row.

**Input zone** (bottom, ~72px) contains quick-action chips, a text input, and a send button.

### 7.2 Agent model selector

Dropdown lists all Groq-hosted models. Recommended defaults:

| Model | Best for |
|---|---|
| `llama-3.3-70b-versatile` | General code reasoning, refactoring, explanation |
| `llama-3.1-8b-instant` | Fast completions, scaffolding, quick fixes |
| `mixtral-8x7b-32768` | Long context, file-spanning analysis |
| `gemma2-9b-it` | Lightweight, low-latency completions |
| `llama3-groq-70b-tool-use` | Agents that call tools (write file, run command) |

### 7.3 Mode toggle

Three segments: Chat, Studio, Agent.

**Chat** — conversational mode. Responses appear inline in the thread. No file mutations without explicit "Apply" confirmation.
**Studio** — compare mode. Response appears in a side-by-side diff view against the current file.
**Agent** — autonomous mode. The agent can write files, run terminal commands, and chain multiple steps. Each action is shown as a step card in the thread with Accept / Reject controls.

### 7.4 Agent header row

Each agent in the thread gets a header row: coloured status dot (teal for active, purple for queued, muted for idle), agent name in the dot's colour at 10px/600, model label right-aligned in `text/ghost`.

Default agents: **Code Agent** (teal dot, llama-3.3-70b) handles TypeScript errors, Svelte reactivity, and component patterns. **Route Agent** (purple dot, llama-3.1-8b) scaffolds routes, API endpoints, and file structures. Additional agents can be added via the `+` button in the header zone.

### 7.5 Message bubbles

**User message** — right-aligned. Background `#231C3A`, border `0.5px #3D2F5E`, text `text/primary`. Max-width 93% of panel.

**Agent message** — left-aligned with agent avatar (20px circle, `surface/1` background, coloured SVG hex icon). Background `surface/1` (`#1E2030`), border `0.5px surface/3`.

**Inline code spans** within messages use `text/string` (`#C3E88D`) for string values, `text/type` (`#FF98A4`) for type names, `text/keyword` (`#FCCB67`) for template syntax.

**Code block** inside an agent message — background `surface/deep` (`#171924`), `0.5px surface/border`, border-radius 4px, padding 6px 8px. Font 10.5px monospace. Full syntax highlighting using the Moonlight token set.

### 7.6 Action buttons in agent messages

When an agent proposes a code change or file operation, it appends action buttons below the message:

**↗ Apply fix / Insert / Extract** — primary button style (`accent/moon` background, `surface/base` text). Applies the suggested change directly into the open file.
**Copy** — secondary button style. Copies the code block to clipboard.
**Dismiss** — ghost button. Collapses the suggestion without applying.

### 7.7 Writing indicator

When an agent is generating a response, the last message shows a blinking `●` dot in the agent's colour followed by a short status label in `text/secondary` (e.g., "writing file", "thinking", "running tool"). The dot uses the `.blink` animation: `opacity` toggling at 0.9s step-end.

### 7.8 Quick-action chips

Five chips sit above the input: "Fix error", "Add types", "Explain", "Write test", "Refactor". Each chip is `surface/1` background, `0.5px surface/border`, `text/secondary`, border-radius 2px, font 9.5px. On click, the chip text is pre-filled into the input and sent immediately.

### 7.9 Input row

Text input: `surface/1` background, full width minus send button. Send button: 30px height, `accent/moon` background, `↑` glyph in `surface/base`, border-radius 4px.

Below the input: a Groq attribution line — hex logo icon + "Powered by Groq · console.groq.com" in `text/ghost` at 9px — and a live latency badge in `accent/teal` style showing the last response time in milliseconds.

### 7.10 Context awareness

The active agent automatically receives:
- The content of the currently focused file
- The current cursor position and selection
- The active diagnostics list from the LSP
- The project framework (SvelteKit / Next.js / Godot) detected from `package.json` or `project.godot`

This context is injected into every request silently. Users do not need to paste code manually.

---

## 8. Integrated terminal

The terminal panel lives at the bottom of the editor column, above the status bar. It is a full PTY (pseudoterminal) running the user's default shell, not a simulated output pane.

### 8.1 Panel structure

**Tab bar** (28px) — horizontal tabs for Terminal, Problems, Output, and Debug Console. Active tab has a `1.5px accent/moon` bottom border. Tabs also show a badge count for Problems when diagnostics exist (badge style: `status/error` variant). Right side: "+ bash" and "⊕ zsh" shell launch buttons, a close `×` button.

**Terminal viewport** — scrollable, background `surface/deep` (`#171924`), padding 8px 14px, font 11px monospace, line-height 1.75. The blinking cursor is a 7×12px block in `accent/moon` using the `.blink` animation.

### 8.2 Terminal colour mapping

The terminal uses a custom 16-colour ANSI palette tuned to the Moonlight theme:

| ANSI role | Hex | Usage |
|---|---|---|
| Black (normal) | `#1B1D2B` | Background fills |
| Red | `#FF98A4` | Errors, stderr |
| Green | `#C3E88D` | Success output |
| Yellow | `#FCCB67` | Warnings |
| Blue | `#82AAFF` | Paths, URLs |
| Magenta | `#C099FF` | Special output |
| Cyan | `#4FD6BE` | Highlights, `✓` confirmations |
| White | `#C8D3F5` | Default text |
| Bright Black | `#3D4468` | Comments, muted lines |
| Bright Red | `#FF5370` | Fatal errors |
| Bright Green | `#91B859` | Build success |
| Bright Yellow | `#FFD580` | Active warnings |
| Bright Blue | `#82AAFF` | Folder names |
| Bright Magenta | `#C792EA` | Secondary purple |
| Bright Cyan | `#89DDFF` | Secondary cyan |
| Bright White | `#FFFFFF` | High-contrast labels |

### 8.3 Shell prompt format

The default Oxide shell prompt renders as:

```
oxide@web:~/project_name $
```

Segments: `oxide@web` in `text/ghost`, `:` separator in `text/muted`, `~/project_name` in `accent/sky`, ` $` in `text/muted`. The prompt is set automatically from the project root when a workspace is opened.

### 8.4 Terminal tabs

Multiple terminal sessions are supported. Each tab appears in the terminal tab bar with a `×` close button. New sessions: "+ bash" opens a bash PTY; "⊕ zsh" opens zsh. Tab labels show the shell name and an optional custom label set by the user. Active session tab uses `accent/moon` bottom border; inactive in `text/muted`.

### 8.5 Problems tab

Shows all LSP diagnostics from the open workspace in a filterable list. Each row: severity icon (`●` in `status/error` or `status/warning`), file path in `text/secondary`, line:col in `text/muted`, message in `text/primary`. Clicking a row navigates the editor to that file and line. Sorted by severity (errors first), then by file.

### 8.6 Output tab

Streams stdout from build tools, test runners, and framework dev servers. Output is colour-coded using the ANSI palette. Sections are delimited by a faint `surface/border` horizontal rule with a small label (e.g., "pnpm dev started at 14:32"). The output is read-only — no input prompt.

### 8.7 Debug Console tab

Displays runtime debug output when the Oxide debugger is attached to a process. Supports REPL-style expression evaluation. Input line at the bottom uses the same style as the terminal input. Output from `console.log`, `print()`, and GDScript `print()` calls is captured here when the dev server is launched via Oxide's run command.

### 8.8 Resize behaviour

The terminal panel is resizable by dragging the top edge of its tab bar. Minimum height: 80px (shows one line of output). Maximum height: 50% of the editor column. The resize handle is 4px tall, background `surface/3` on hover, invisible at rest.

---

## 9. Motion & interaction

Oxide is a near-zero animation environment.

| Interaction | Transition |
|---|---|
| Tab switch | Instant (0ms) |
| Sidebar resize | Instant (live drag) |
| File tree expand/collapse | Instant |
| Terminal panel resize | Instant (live drag) |
| Agents panel collapse/expand | `width 120ms ease-out` |
| Badge appearance | `opacity 120ms ease-out` |
| Hover state (rows, icons) | `background 80ms ease` |
| Tooltip appear | `opacity 100ms ease` |
| HMR reload flash | `opacity 200ms ease-out` on canvas |
| Notification slide-in | `transform 160ms ease-out` from bottom |
| Agent writing indicator | `opacity` toggle 0.9s step-end (`.blink`) |

No bounce, spring, or overshoot easing anywhere.

---

## 10. Accessibility

- Minimum contrast ratio 4.5:1 for all body text. `text/primary` on `surface/2` measures 9.8:1.
- Active tab state: conveyed by top border AND dot indicator — never colour alone.
- Agent status: conveyed by coloured dot AND label text — never colour alone.
- Terminal errors: conveyed by colour AND `●` prefix glyph — never colour alone.
- Focus rings: `2px solid #C099FF`, 2px offset on all keyboard-focusable elements.
- Reduced motion: all transitions wrapped in `@media (prefers-reduced-motion: no-preference)`.
- Font size floor: 11px everywhere except section headers (9px minimum, only in chrome).
- `.env` value blur: toggled by explicit click with `aria-label="Reveal value"`.
- Agent code blocks: include a hidden `aria-label` summarising the code block language and line count.
- Terminal: keyboard-accessible with full readline key bindings. Screen reader announces new output lines with `aria-live="polite"`.

---

*Oxide Design Document — internal reference. Subject to change between minor versions. Color tokens are defined separately in `oxide-tokens.md`.*
