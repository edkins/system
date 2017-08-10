mod terminal;
mod vimkeys;
mod keyboard;

fn main() {
    let mut t = terminal::new();
    let mut vk = vimkeys::new(t);
    let mut kb = keyboard::new(vk);
    kb.run();
}
