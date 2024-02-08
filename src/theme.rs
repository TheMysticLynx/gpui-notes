use gpui::{rgb, Rgba};

pub struct Theme {
    e0: Rgba,
    e1: Rgba,
    e2: Rgba,
    e3: Rgba,
    borders: Rgba,
}

impl Theme {
    pub fn default() -> Self {
        Self {
            e0: rgb(0x0E1015),
            e1: rgb(0x1F2126),
            e2: rgb(0x313337),
            e3: rgb(0x3F4146),
            borders: rgb(0x3F4043),
        }
    }

    pub fn e0(&self) -> Rgba {
        self.e0
    }

    pub fn e1(&self) -> Rgba {
        self.e1
    }

    pub fn e2(&self) -> Rgba {
        self.e2
    }

    pub fn e3(&self) -> Rgba {
        self.e3
    }

    pub fn borders(&self) -> Rgba {
        self.borders
    }
}
