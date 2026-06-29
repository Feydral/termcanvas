# termcanvas 🎨

A lightweight Rust library for rendering pixels directly in the terminal. Colors are written into a pixel buffer each frame, which is then flushed to the screen.

> **Note:** Your terminal must support ANSI true color (24-bit). Most modern terminals do — Windows Terminal, iTerm2, Alacritty, Kitty, and WezTerm all work.

## Quickstart

```rust
use termcanvas::prelude::*;
use crossterm::event::KeyCode;

let mut canvas = Canvas::new();
let mut input = Input::new();
let font = Font::load_from_file("assets/default.tcfont");

loop {
    input.update().unwrap();
    if input.is_key_down(KeyCode::Esc) { break; }

    canvas.clear(mathi::rgb_to_u32(0, 0, 0));
    canvas.set_pixel(10, 10, mathi::rgb_to_u32(255, 100, 0));
    canvas.draw(&font).at(20, 10).text("Hello!");
    canvas.render();
}

canvas.end();
```

## Features

- 🖼️ **Pixel rendering** — set and read individual pixels on a canvas that adapts to your terminal size ([details](#pixel-rendering))
- ✏️ **Bitmap font system** — render text and numbers using two included fonts, with a fluent draw API ([details](#font-rendering))
- ⌨️ **Input handling** — per-frame key-down, key-held, and key-up states via the Kitty keyboard protocol ([details](#input))

## How it works

Each terminal cell is rendered as two pixels stacked vertically, using the Unicode block character `▀` with an ANSI true-color foreground (top pixel) and background (bottom pixel). A terminal of 80×24 cells therefore gives a canvas of 80×48 pixels.
To keep rendering fast, color escape codes are only emitted when the color actually changes compared to the previous cell. The entire frame is assembled into a single byte buffer and written in one call, wrapped in synchronized output (`?2026h/l`) to eliminate flickering.

## Usage

### Pixel rendering

Colors are passed as `0xRRGGBBAA` packed `u32` values. You can construct them manually or use the included `mathi::rgb_to_u32` helper. Call `canvas.clear(color)` at the start of each frame to reset the buffer and `canvas.render()` at the end to flush it to the terminal. When the loop exits, it is recommended to call `canvas.end()` to restore the terminal to its original state (cursor visibility, alternate screen, etc.).

```rust
use termcanvas::prelude::*;
use crossterm::event::KeyCode;

let mut canvas = Canvas::new();
let mut input = Input::new();

let red = mathi::rgb_to_u32(255, 0, 0);
let green = mathi::rgb_to_u32(0, 255, 0);
let blue = mathi::rgb_to_u32(0, 0, 255);

loop {
    input.update().unwrap();
    if input.is_key_down(KeyCode::Esc) { break; }

    canvas.clear(mathi::rgb_to_u32(0, 0, 0));

    canvas.set_pixel(10, 10, red);
    canvas.set_pixel(11, 10, green);
    canvas.set_pixel(12, 10, blue);

    let color = canvas.get_pixel(10, 10); // returns 0xRRGGBBAA

    canvas.render();
}

canvas.end(); // restores cursor, clears alternate screen
```

### Font rendering

Two bitmap fonts are included and ready to use — `default` and `default_bold`. Load them from the `assets/` folder and use the fluent draw builder to place text at any position and color.

Text is drawn with `.text()`. For numbers, use `.uint()`, `.int()`, or `.float()` to render values directly without formatting them yourself.

```rust
use termcanvas::prelude::*;

let font = Font::load_from_file("assets/default.tcfont");
let font_bold = Font::load_from_file("assets/default_bold.tcfont");

let white = mathi::rgb_to_u32(255, 255, 255);
let gray = mathi::rgb_to_u32(180, 180, 180);

canvas.draw(&font_bold).at(10, 10).color(white).text("Console Canvas");

canvas.draw(&font).at(10, 30).color(gray).text("score:");
canvas.draw(&font).at(10, 40).color(white).uint(4200);

canvas.draw(&font).at(10, 60).color(gray).text("temperature:");
canvas.draw(&font).at(10, 70).color(white).float(98.6, 1, false); // 1 decimal, no forced sign

canvas.draw(&font).at(10, 90).color(gray).text("delta:");
canvas.draw(&font).at(10, 100).color(white).int(42, true); // always_show_sign: true -> "+42"
```

**Alignment** is set with `.align(Align::Left)` (default), `.align(Align::Right)` or `.align(Align::Middle)`. With `Align::Left`, the `x`/`y` coordinates mark the top-left corner of the text. With `Align::Right`, they mark the top-right corner — useful for right-aligning numbers at a fixed column without calculating their width yourself. With `Align::Middle`, `x`/`y` mark the top-center — useful for centering headings or labels, for example at `width / 2` to center across the full canvas.

### Input

Call `input.update()` once at the start of each frame to process all pending events. Key state is tracked across three separate sets, each reset every frame:

| Method | Returns `true` when… |
|---|---|
| `is_key_down(key)` | The key was pressed this frame (fires once) |
| `is_key_pressed(key)` | The key is currently held down |
| `is_key_up(key)` | The key was released this frame (fires once) |

```rust
input.update().unwrap();

if input.is_key_down(KeyCode::Space) {
    // triggered once on press
}
if input.is_key_pressed(KeyCode::Left) {
    // true every frame while held
}
if input.is_key_up(KeyCode::Space) {
    // triggered once on release
}
```

> **Platform note:** On Linux and macOS, the Kitty keyboard protocol is required and must be supported by the terminal for `Input` to initialize. If it is not available, the program will panic on startup — only `Input` is affected, the canvas itself has no such requirement. On Windows, the Kitty protocol is not used and input works out of the box.

## Project structure

```
src/
  lib.rs
  prelude.rs
  canvas.rs        # Canvas: pixel buffer, frame render, terminal resize
  canvas/
    draw.rs        # DrawBuilder fluent API, text and number rendering
    font.rs        # Font: .tcfont file loader and glyph parser
  input.rs         # Input: raw mode, Kitty protocol, per-frame key state
  math.rs          # Re-exports mathf, mathi, noise
  math/
    mathf.rs       # Float math helpers
    mathi.rs       # Integer math helpers
    noise.rs       # Noise functions
assets/
  default.tcfont       # Regular bitmap font
  default_bold.tcfont  # Bold bitmap font
```

## Dependencies

- [`crossterm`](https://github.com/crossterm-rs/crossterm) — terminal control and input
- [`glam`](https://github.com/bitshifter/glam-rs) — vector math
