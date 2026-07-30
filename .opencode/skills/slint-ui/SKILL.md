---
name: slint-ui
description: Slint declarative UI markup syntax, callbacks, component composition. Use when writing .slint files.
---

# Slint UI — markup patterns

Slint is a declarative UI toolkit. Single language for markup; logic lives in Rust.

## File extensions

- `.slint` — primary markup
- `.slint?` — template files (Rust bindings generated)

## Basic structure

```slint
// app-window.slint
import { Button } from "std-widgets.slint";
import { GameCard } from "../components/game-card.slint";

export component AppWindow inherits Window {
    in-out property <string> username: "";
    in-out property <[Game]> games: [];
    
    VerticalLayout {
        Text { text: "Hello, " + root.username; }
        for game in root.games: GameCard {
            game: game;
        }
    }
}
```

## Properties

```slint
// Property declarations
in property <string> title: "default";      // Input only (parent → child)
out property <int> count: 0;                // Output only (child → parent)
in-out property <bool> enabled: true;       // Two-way binding

// Property binding (reactive)
in property <string> name: "world";
Text { text: "Hello, " + root.name + "!"; }  // Updates when name changes
```

## Callbacks

```slint
export component Button {
    in property <string> label: "Click me";
    callback clicked();
    
    area := TouchArea {
        clicked => { root.clicked(); }
    }
    Text { text: root.label; }
}
```

Rust side:
```rust
let button = Button::new()?;
button.on_clicked(|_| {
    println!("clicked!");
});
```

## Layouts

```slint
// Vertical stacking
VerticalLayout {
    spacing: 8px;
    Text { ... }
    Button { ... }
}

// Horizontal stacking
HorizontalLayout {
    spacing: 4px;
    Text { ... }
    Button { ... }
}

// Grid (manual)
GridLayout {
    spacing: 8px;
    Row {
        Text { ... }
        Button { ... }
    }
}
```

## Lists

```slint
// Static list — ALL items render. Stay <50 items.
ListView {
    for item in root.items: Rectangle {
        Text { text: item.name; }
    }
}

// Manual virtualization for >100 items
VerticalLayout {
    for item in root.items[0..min(root.items.length, 50)]: GameCard {
        game: item;
    }
}
// Listen to scroll event and update the slice.
```

## Conditionals + states

```slint
if root.is_logged_in: Text { text: "Welcome"; }
if !root.is_logged_in: LoginForm {}

// Ternary in expressions
Text { text: root.is_loading ? "Loading..." : "Library"; }
```

## Animations

```slint
Rectangle {
    width: 100px;
    animate width { duration: 200ms; }
    // Hover-driven animation
    states [
        hover: {
            width: 120px;
        }
    ]
    area := TouchArea {
        mouse-cursor: pointer;
        hovered => {
            root.hover = !root.hover;
        }
    }
}
```

## Images

```slint
Image {
    source: @image-url("icons/cover.png");
    image-fit: cover;
    width: 200px;
    height: 300px;
}

// From Rust
Image {
    source: root.cover_image;  // slint::Image
}
```

```rust
let image = slint::Image::load_from_path(&PathBuf::from("cover.png"))?;
ui.set_cover_image(image);
```

## Theming

```slint
// Define in a single style file
export global Theme {
    out property <color> bg: #1e1e1e;
    out property <color> fg: #ffffff;
    out property <color> accent: #00ff88;
}

// Use anywhere
Text { color: Theme.fg; }
```

## Don't

- Don't put business logic in `.slint` — emit callbacks, handle in Rust.
- Don't use `Timer` for one-shot animations — use `Animation` properties.
- Don't block on network calls from Rust event handlers — use tokio tasks.
- Don't use `Image::load_from_url` synchronously — preload via background task.
- Don't use `for x in items: SomeComponent { ... }` for >100 items without virtualization.
