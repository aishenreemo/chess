pub struct Cache {
    pub focused_square: Option<(usize, usize)>,
}

impl Cache {
    pub fn init() -> Self {
        Self {
            focused_square: None,
        }
    }
}
