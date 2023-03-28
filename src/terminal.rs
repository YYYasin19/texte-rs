pub struct Size {
    pub w: u16,
    pub h: u16,
}

pub struct Terminal {
    size: Size, // not pub, nobody can modify size
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                w: size.0,
                h: size.1,
            }
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}