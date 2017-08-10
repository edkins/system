mod terminal;
mod keyboard;

fn main() {
    terminal::create().R(String::from("stuff"));
}
