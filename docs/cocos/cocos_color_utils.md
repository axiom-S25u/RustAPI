# Cocos Color Utils

Color helpers for Cocos2d-x colors.

### Functions
- `cocos::color::invert3b(r: u8, g: u8, b: u8) -> (u8, u8, u8)`: Inverts a ccColor3B.
- `cocos::color::lighten3b(r: u8, g: u8, b: u8, amount: i32) -> (u8, u8, u8)`: Lightens a ccColor3B by amount.
- `cocos::color::darken3b(r: u8, g: u8, b: u8, amount: i32) -> (u8, u8, u8)`: Darkens a ccColor3B by amount.
- `cocos::color::cc3b_from_hex(hex: &str) -> Option<(u8, u8, u8)>`: Parses a hex string to a ccColor3B.
- `cocos::color::cc4b_from_hex(hex: &str) -> Option<(u8, u8, u8, u8)>`: Parses a hex string to a ccColor4B.
- `cocos::color::invert4b(r: u8, g: u8, b: u8, a: u8) -> (u8, u8, u8, u8)`: Inverts a ccColor4B.
- `cocos::color::to3b(r: u8, g: u8, b: u8, a: u8) -> (u8, u8, u8)`: Converts a ccColor4B to ccColor3B.
- `cocos::color::to4b(r: u8, g: u8, b: u8, alpha: u8) -> (u8, u8, u8, u8)`: Converts a ccColor3B to ccColor4B.
- `cocos::color::to4f(r: u8, g: u8, b: u8, a: u8) -> (f32, f32, f32, f32)`: Converts a ccColor4B to ccColor4F (floats between 0.0 and 1.0).
- `cocos::color::cc3b_to_hex(r: u8, g: u8, b: u8) -> String`: Formats ccColor3B as a hex string.
- `cocos::color::cc4b_to_hex(r: u8, g: u8, b: u8, a: u8) -> String`: Formats ccColor4B as a hex string.

### Example - Conversions & Invert
```rust
use rustapi::bindings::geode::cocos;

fn tweak_colors() {
    let original = (255, 100, 50, 255);
    
    // convert to float color (ccColor4F)
    let float_color = cocos::color::to4f(original.0, original.1, original.2, original.3);
    println!("Float color: {:?}", float_color);
    
    // invert color
    let inverted = cocos::color::invert4b(original.0, original.1, original.2, original.3);
    
    // format to hex string
    let hex = cocos::color::cc4b_to_hex(inverted.0, inverted.1, inverted.2, inverted.3);
    println!("Inverted hex: {}", hex);
}
```

### Example - Modify and Lighten/Darken
```rust
use rustapi::bindings::geode::cocos;

fn adjust_brightness() {
    if let Some(gold) = cocos::color::cc3b_from_hex("#FFD700") {
        let brighter = cocos::color::lighten3b(gold.0, gold.1, gold.2, 30);
        let darker = cocos::color::darken3b(gold.0, gold.1, gold.2, 50);
        
        println!("Gold hex: {}", cocos::color::cc3b_to_hex(gold.0, gold.1, gold.2));
        println!("Brighter hex: {}", cocos::color::cc3b_to_hex(brighter.0, brighter.1, brighter.2));
    }
}
```

### cpp side

you dont need cpp side because all color helpers work directly in rust

