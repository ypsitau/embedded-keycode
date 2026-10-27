//use std::fmt::Display as _;
use embedded_keycode as keycode;

fn main() {
    let modifier = keycode::Modifier::default();
    println!("{}", keycode::Key::A(modifier.into()));
    println!("{}", keycode::Key::B(modifier.into()));
    println!("{}", keycode::Key::C(modifier.into()));
    println!("{}", keycode::Key::D(modifier.into()));
    println!("{}", keycode::Key::E(modifier.into()));
    println!("{}", keycode::Key::F(modifier.into()));
    println!("{}", keycode::Key::G(modifier.into()));
    println!("{}", keycode::Key::H(modifier.into()));
    println!("{}", keycode::Key::I(modifier.into()));
    println!("{}", keycode::Key::J(modifier.into()));
    println!("{}", keycode::Key::A(modifier.shift_l().into()));
    println!("{}", keycode::Key::A(modifier.shift_l().into()));
}
