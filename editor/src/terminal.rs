#![allow(non_snake_case)]

pub struct Term {
}

impl Term {
    pub fn R( &mut self, text : String ) {
        print!("{}",text)
    }
}

pub fn create() -> Term {
    return Term{}
}
