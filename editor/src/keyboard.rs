use vimkeys::VimKeys;
use std::io;
use std::io::Read;

pub struct Keyboard {
    vk : VimKeys
}

impl Keyboard {
    pub fn run( &mut self ) {
        for c in io::stdin().bytes() {
            self.vk.char(char::from(c.unwrap()));
        }
    }
}

pub fn new( mut vk : VimKeys ) -> Keyboard {
    return Keyboard{vk}
}
