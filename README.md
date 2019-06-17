# Rusty Object Notation in Debug format

[![Build Status](https://travis-ci.org/ron-rs/ron.svg?branch=master)](https://travis-ci.org/ron-rs/ron)
[![Crates.io](https://img.shields.io/crates/v/ron.svg)](https://crates.io/crates/ron)
[![Docs](https://docs.rs/ron/badge.svg)](https://docs.rs/ron)
[![Gitter](https://badges.gitter.im/ron-rs/ron.svg)](https://gitter.im/ron-rs/ron)

ROND is a simple readable data serialization format that looks similar format produced by Debug (AKA {:?}) formatter.
It's designed to support all of [Serde's data model](https://serde.rs/data-model.html), so
structs, enums, tuples, arrays, generic maps, and primitive values.

## Example

```
GameConfig { // optional struct name
    window_size: (800, 600),
    window_title: "PAC-MAN",
    fullscreen: false,
    
    mouse_sensitivity: 1.4,
    key_bindings: {
        "up": Up,
        "down": Down,
        "left": Left,
        "right": Right,
        
        // Uncomment to enable WASD controls
        /*
        "W": Up,
        "A": Down,
        "S": Left,
        "D": Right,
        */
    },
    
    difficulty_options: {
        start_difficulty: Easy,
        adaptive: false,
    },
}
```

## Why ROND?

Because Rust produces debugging dumps in this format, which needed to be parsed back sometimes.

### Example in ROND

```rust
Scene { // class name is optional
    materials: { // this is a map
        "metal": {
            reflectivity: 1.0,
        },
        "plastic": {
            reflectivity: 0.5,
        },
    },
    entities: [ // this is an array
        {
            name: "hero",
            material: "metal",
        },
        {
            name: "monster",
            material: "plastic",
        },
    ],
}
```

Here are the general rules to parse the heterogeneous structures:

| class is named? | fields are named? | what is it?               | example             |
| --------------- | ------------------| ------------------------- | ------------------- |
| no              | no                | tuple                     | `(a, b)`            |
| yes/no          | no                | tuple struct              | `Name(a, b)`        |
| yes             | no                | enum value                | `Variant(a, b)`     |
| yes/no          | yes               | struct                    | `{f1: a, f2: b,}`   |

## License

ROND is dual-licensed under Apache-2.0 and MIT.

