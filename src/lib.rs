//! Keycode handling for embedded systems
//!
//! Each key data is represented by enumeration variant `Key::Name(modifier)`.
//!
//! - `Name` is a key name that corresponds to the definitions in Linux's `input-event-codes.h`.
//! - `modifier` is a `Modifier` struct that represents the state of modifier keys (Ctrl, Shift, Alt, Meta).
//!
//! | Code                                             | Description                           |
//! |--------------------------------------------------|---------------------------------------|
//! | `Key::Left(Modifier::default().into())`          | Left arrow key                        |
//! | `Key::A(Modifier::default().into())`             | A key                                 |
//! | `Key::Up(Modifier::default().ctrl().into())`     | Up arrow key with Ctrl modifier       |
//! | `Key::Space(Modifier::default().shift().into())` | Space key with Shift modifier         |
//!
//! When key input comes from a serial interface, printable characters are represented in ASCII or UTF-8,
//! which are generated from the raw key code by processing shift and control modifier states.
//! Use `Key::CookedChar()` and `Key::CookedCtrl()` to handle such input.
//!
//! | Code                        | Description             |
//! |----------------------------|-------------------------|
//! | `Key::CookedChar('A')`      | Cooked character 'A'    |
//! | `Key::CookedChar('!')`      | Cooked character '!'    |
//! | `Key::CookedCtrl(0x01)`     | Cooked character Ctrl-A |
//!
//! Use pattern matching to handle key inputs.
//!
//! ```rust
//! use embedded_keycode::*;
//! fn handle_key(key: Key) {
//!     match key {
//!         Key::CookedChar(ch) => { /* handle cooked character */ },
//!         Key::CookedCtrl(ch) => { /* handle cooked control character */ },
//!         Key::Left(attr) => { /* handle left key */ },
//!         Key::Right(attr) => { /* handle right key */ },
//!         _ => { /* handle other keys */ },
//!     }
//! }
//! ```
//!
//! `attr` represents the modifier attributes associated with a key.
//!
//! | Method                 | Description                               |
//! |------------------------|-------------------------------------------|
//! | `attr.is_shift()`      | Checks if any Shift key is pressed        |
//! | `attr.is_shift_l()`    | Checks if the left Shift key is pressed   |
//! | `attr.is_shift_r()`    | Checks if the right Shift key is pressed  |
//! | `attr.is_ctrl()`       | Checks if any Ctrl key is pressed         |
//! | `attr.is_ctrl_l()`     | Checks if the left Ctrl key is pressed    |
//! | `attr.is_ctrl_r()`     | Checks if the right Ctrl key is pressed   |
//! | `attr.is_alt()`        | Checks if any Alt key is pressed          |
//! | `attr.is_alt_l()`      | Checks if the left Alt key is pressed     |
//! | `attr.is_alt_r()`      | Checks if the right Alt key is pressed    |
//! | `attr.is_meta()`       | Checks if any Meta key is pressed         |
//! | `attr.is_meta_l()`     | Checks if the left Meta key is pressed    |
//! | `attr.is_meta_r()`     | Checks if the right Meta key is pressed   |
#![no_std]


mod keycode;
pub use keycode::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modifier() {
        let attr = Modifier::default().ctrl().shift();
        assert!(attr.is_ctrl());
        assert!(attr.is_shift());
        assert!(!attr.is_alt());
    }
}
