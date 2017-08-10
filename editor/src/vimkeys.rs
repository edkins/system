use terminal::Terminal;

pub struct VimKeys {
    t : Terminal
}

impl VimKeys {
    pub fn char( &mut self, c : char ) {
        self.t.R(c.to_string())
    }
}

pub fn new( mut t : Terminal ) -> VimKeys {
    return VimKeys{t}
}
