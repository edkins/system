#![allow(non_snake_case)]

pub struct Terminal {
}

impl Terminal {
    pub fn R( &mut self, text : String ) {
        print!("{}",text)
    }
}

pub fn new() -> Terminal {
    return Terminal{}
}
