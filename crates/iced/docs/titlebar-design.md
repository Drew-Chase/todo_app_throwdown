# Titlebar Design (iced 0.14)

Step-by-step design for a custom, self-contained titlebar that "handles its own
events". Written for `crates/iced` (`iced_todo_app`), iced 0.14.0.

All API locations referenced below were verified against the iced 0.14 sources
in the local cargo registry (`~/.cargo/registry/src/index.crates.io-*/iced*`).

---

## 0. The mental model (30 seconds)

iced is Elm-architecture. Nothing in the UI acts on its own:

```
mouse press on titlebar
        │
        ▼
widget mails a note:  TitlebarMessage::StartDragWindow
        │
        ▼
TodoApp::update() reads the note          ← the ONLY place that decides anything
        │
        ▼
update returns a Task ("iced, please ask the OS to drag the window")
        │
        ▼
view() re-runs from the (unchanged) state → same pixels
```

Two rules that make this click:

1. **Widgets never do side effects.** Their whole job is: *recognize a gesture,
   publish a Message*. If you find yourself wanting to call `window::drag`
   from inside a widget — that is the Elm voice saying "publish a note instead".
2. **`view()` is a photograph, not a living object.** It is re-run from scratch
   after every message. Widgets don't persist; your `TodoApp` struct persists.

## 1. Where the code stands today

| File | State |
|---|---|
| `src/app/titlebar.rs` | Stub: `Titlebar` struct (empty), `TitlebarMessage` enum already defined (`Close`, `ToggleMaximize`, `ToggleMinimize`, `StartDragWindow`, `EndDragWindow`), incomplete `impl Widget<TitlebarMessage, >` |
| `src/app/mod.rs` | `TodoApp` (empty struct), `TodoAppMessage` (empty enum, `match message {}`), `view` returns empty `row![]`, `window_settings()` |
| `src/lib.rs` | `iced::application(...)` builder; `.window(TodoApp::window_settings())` |

Key facts already in place:

- `decorations: !cfg!(target_os = "windows")` (mod.rs:40) — **the native bar is
  gone on Windows only**. Linux/macOS keep native decorations (macOS uses
  `fullsize_content_view`). So the titlebar is a Windows concern today.
- `undecorated_shadow: true` on Windows (mod.rs:45) — you keep the DWM drop
  shadow, good.
- The `advanced` feature is already enabled in Cargo.toml:13, which is required
  for `iced::advanced::Widget` (iced-0.14.0/src/advanced.rs:27).
- `iced_winit` is an unconditional dependency of the facade
  (iced-0.14.0/Cargo.toml:169) — windowing tasks (`window::drag` etc.) always
  link; no extra feature needed.

**Important correction to common iced knowledge: iced 0.14 has no
`window::Id::MAIN`.** `window::Id` only has `unique()`
(iced_core-0.14.0/src/window/id.rs:12-16). For a single-window app, obtain the
Id with `iced::window::latest() -> Task<Option<Id>>`
(iced_runtime-0.14.0/src/window.rs:294) and chain the action with
`Task::then` (iced_runtime-0.14.0/src/task.rs:128).

## 2. Architecture

One message type per layer, one direction of flow:

```
Titlebar widget (src/app/titlebar.rs)
   publishes:   TitlebarMessage            — gestures only, no side effects
        │
TodoApp::update (src/app/mod.rs)
   matches:     TodoAppMessage::Titlebar(inner)
   returns:     window::latest().then(|id| window::drag(id))   ← side effects live HERE
```

Ownership decisions:

- `TitlebarMessage` stays in `titlebar.rs` (it already is). It is a *gesture
  vocabulary*: press/double-click/close/maximize/minimize. Nothing else.
- The *maximized* flag (needed to pick the restore vs. maximize icon) is app
  state, not widget state. Widgets are stateless descriptions.
- Window actions appear in exactly one place: `TodoApp::update`. That makes
  them greppable and testable.

## 3. Route A — composition (recommended first)

No custom widget at all. A function that builds a `Row`, and one new message
variant. Everything below is a sketch to build from, not a paste-in.

### Step 1 — wrap the message

Add one variant to `TodoAppMessage` in `src/app/mod.rs`:

- `Titlebar(TitlebarMessage)`

Side effect to be aware of: `update` currently does `match message {}` against
an *empty* enum, which compiles. Adding a variant makes that match
non-exhaustive — the compiler will point you at exactly the place you need to
touch. That is the Elm arch working for you.

### Step 2 — the view function (in `titlebar.rs`)

A `pub fn view() -> Element<'_, TitlebarMessage>` that builds:

```
row![
    // left: app icon + "Sift" text (plain widgets, no events needed)
    // fill: mouse_area stretched over the remaining space
    // right: three buttons
]
```

The drag surface is `iced::widget::mouse_area` stretched with
`Length::Fill`, configured with the builders that exist in 0.14
(iced_widget-0.14.2/src/mouse_area.rs):

| Builder | Line | Use for |
|---|---|---|
| `on_press(msg)` | :38 | start OS drag |
| `on_double_click(msg)` | :61 | toggle maximize |
| `on_enter` / `on_exit` | :106/:120 | hover styling, optional |
| `on_right_press(msg)` | :68 | optional: window system menu → `window::show_system_menu` (iced_runtime window.rs:443) |

The three window buttons are plain `iced::widget::button`s, each with a
`TitlebarMessage` (`ToggleMinimize`, `ToggleMaximize`, `Close`). Icons: reuse
the svg icon helper in `src/app/theme.rs`; colors come from the theme ramps.

### Step 3 — the update arms (in `mod.rs`)

Map each gesture to a window task. The 0.14 functions (iced_runtime-0.14.0/src/window.rs):

| Gesture | Task | Line |
|---|---|---|
| `StartDragWindow` | `window::latest().then(\|opt\| opt.map(window::drag).unwrap_or_else(Task::none))` | :294, :299 |
| `ToggleMaximize` | same shape with `window::toggle_maximize(id)` | :400 |
| `ToggleMinimize` | same shape with `window::minimize(id, true)` | :364 |
| `Close` | same shape with `window::close(id)` | :284 |

The `latest().then(...)` dance exists because there is no `Id::MAIN` (see §1).
All four tasks return `Task<T>` and are infallible, so no error plumbing.

### Step 4 — include it in the app view

In `TodoApp::view`, add the titlebar as the first child of a `column![]`, but
**gate it the same way decorations are gated** (mod.rs:40):

- Windows: include the titlebar.
- Linux/macOS: skip it — the OS draws a bar, and drawing your own means two.

A `#[cfg(target_os = "windows")]` on the titlebar child (mirroring the
`platform_specific` blocks already in `window_settings`) is the idiomatic
route in this crate.

### Route A gotcha: double-click vs. press

On a double-click, `on_press` fires *twice* and `on_double_click` fires once.
The first press starts the OS drag loop, which typically swallows the second
click. In practice on Windows this means: drag-on-press works, and
double-click-maximize works on the *second* attempt after a drag. If that
feels wrong, the fix is Route B, where you control the timing yourself
(detect press → wait → classify as click vs. double-click → publish the right
message). Windows convention is drag-on-press, so v1 can ship as-is.

## 4. Route B — finish the custom `Widget` (what `titlebar.rs` started)

Use this when you want custom painting (gradient bar, hover glow) or your own
hit-testing/click-classification. The existing stub needs reshaping:

### 4.1 Fix the generic signature

`Widget` takes three parameters: `Widget<Message, Theme, Renderer>`
(iced_core-0.14.0/src/widget.rs:37). The stub's `impl Widget<TitlebarMessage, >`
cannot compile. The idiomatic shape is to make the widget generic over the
app message and store a mapper:

- `pub struct Titlebar<Message> { on_event: Box<dyn Fn(TitlebarMessage) -> Message> }`
- `impl<Message> Widget<Message, iced::Theme, iced::Renderer> for Titlebar<Message>`

This is exactly how core widgets like `button` are built (they hold
`Option<Message>`; you hold a closure because you emit several messages).

### 4.2 The trait-method checklist

| Method | Must do | Notes |
|---|---|---|
| `size()` / `size_hint()` | `Size(Length::Fill, Length::Fixed(32.0))`-ish | height from theme metrics |
| `layout()` | compute one node filling the given limits | leaf widget; no children |
| `draw()` | paint the bar into `layout.bounds()` using `Theme` ramps | `mouse::Cursor::Available(p)` tells you hover; rebuild the icon highlight from it — no state needed for simple hover |
| `update()` | **the whole point** — see below | default impl is a no-op (widget.rs:112) |
| `mouse_interaction()` | return `mouse::Interaction::Moving` over the bar | gives the OS move cursor |
| `state()` | `tree::State::new()` (default) | add real state only when you need click-vs-double-click timing |

### 4.3 The `update` logic

Signature (iced_core-0.14.0/src/widget.rs:112-123):

```
fn update(&mut self, tree, event: &Event, layout, cursor, renderer,
          clipboard, shell: &mut Shell<Message>, viewport)
```

Inside:

1. `if !cursor.is_over(layout.bounds()) { return; }` — `is_over` is at
   iced_core-0.14.0/src/mouse/cursor.rs:52.
2. Match `Event::Mouse(mouse::Event::ButtonPressed(MouseButton::Left))` →
   `shell.publish((self.on_event)(TitlebarMessage::StartDragWindow))` and
   `shell.capture_event()`.
3. Double-click classification (why Route B is worth it): keep a
   `last_click: Option<Instant>` in a custom state struct stored in the
   `Tree` (via `state()`); if the press is within the OS double-click
   interval, publish `ToggleMaximize` instead of `StartDragWindow` — the drag
   never starts, so the Route A gotcha disappears.
4. `shell.capture_event()` (iced_core-0.14.0/src/shell.rs:49) stops the event
   from reaching widgets below; `publish` (:41) mails the note; both compose.

### 4.4 The buttons

A leaf widget cannot easily host child buttons (that means implementing
`children()`/`operate_on_children()` and diffing `Tree` state by hand —
iced_core widget.rs:85-90 is the surface, but it is fiddly). Recommendation:
keep Route B scoped to the **drag surface**, and compose the three buttons
next to it in the `row![]` exactly like Route A. Best of both: custom
painting/hit-testing where it matters, stock widgets where they already work.

## 5. Side effects & risks checklist

Worked through per the project analysis protocol:

- **Exhaustiveness**: adding `TodoAppMessage::Titlebar` breaks
  `match message {}` — intended, compiler-guided.
- **Close semantics**: `window_settings` has `exit_on_close_requests: true`
  (mod.rs:74). `window::close(id)` bypasses close-request interception; if you
  later want "confirm before quit", route `Close` through a close-request
  *message* path instead of the task. v1: `window::close` is fine.
- **Maximized icon state**: picking restore vs. maximize icon requires knowing
  maximized state. Out of scope for v1 (static icons). Later: app state seeded
  with `window::is_maximized(id) -> Task<bool>` (window.rs:345) and kept fresh
  via `window::resize_events() -> Subscription<(Id, Size)>` (window.rs:249).
- **Resize borders on undecorated Windows windows**: winit provides edge
  hit-testing for undecorated+resizable windows; verify at runtime. Fallback
  if edges are dead: `window::drag_resize(id, Direction)` (window.rs:304).
- **Platform double-titlebar**: titlebar must be cfg-gated like decorations
  are (§3 Step 4) or Linux/macOS get two bars.
- **No allocations in hot paths**: `update` runs on *every* mouse event over
  the bar. Route A's closures are built once in `view` (fine); Route B must
  not allocate per event (no `Vec`/`String`/boxing inside `update`).
- **Threading**: none of this touches shared state; `Task`s run on iced's
  runtime. No locks introduced.

## 6. Verification

```
cargo clippy -p iced_todo_app
cargo run -p iced_todo_app     # or: just iced
```

Manual test script (Windows): drag from empty bar area → moves window;
double-click → toggles maximize; `_`/`□`/`✕` behave; hover over bar shows
move cursor; resize from edges still works; confirm no titlebar on a Linux
build.

## 7. API cheat sheet (all verified, iced 0.14)

| Item | Location |
|---|---|
| `Widget` trait, `update` signature, `state`, `children` | iced_core-0.14.0/src/widget.rs:37, :112, :85 |
| `Shell::{publish, capture_event, request_redraw}` | iced_core-0.14.0/src/shell.rs:41, :49, :66 |
| `mouse::Cursor::is_over` | iced_core-0.14.0/src/mouse/cursor.rs:52 |
| `mouse_area::{on_press, on_double_click, on_enter, on_exit, on_right_press}` | iced_widget-0.14.2/src/mouse_area.rs:38, :61, :106, :120, :68 |
| `window::{latest, drag, drag_resize, close, minimize, toggle_maximize, is_maximized, resize_events, show_system_menu}` | iced_runtime-0.14.0/src/window.rs:294, :299, :304, :284, :364, :400, :345, :249, :443 |
| `Task::{then, chain}` | iced_runtime-0.14.0/src/task.rs:128, :159 |
| No `Component` trait; no `window::Id::MAIN` | absent by design in 0.14 |

## 8. Implementation order

1. `TodoAppMessage::Titlebar(TitlebarMessage)` variant (Step 1) — compile.
2. `titlebar::view()` with `mouse_area` + buttons (Step 2) — compile.
3. Update arms with `latest().then(...)` (Step 3) — compile.
4. cfg-gate into `TodoApp::view` (Step 4) — compile, run, manual tests.
5. Ship Route A. Only then consider Route B for painting/timing.

Each step leaves the crate green — no big-bang rewrite of the stub.
