const COLOR_RANGE: u16 = 255;

pub struct Colour
{
    pub red:   u16,
    pub green: u16,
    pub blue:  u16,
    pub alpha: u16
}

pub struct Color
{
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Colour
{
    pub fn new (red: u16, green: u16, blue: u16, alpha: u16) -> Self {
        Self { red, green, blue, alpha }
    }

    pub fn converted_to_color(&self) -> Color
    {
        let r = (self.red / COLOR_RANGE) as f64;
        let g = (self.green / COLOR_RANGE) as f64;
        let b = (self.blue / COLOR_RANGE) as f64;
        let a = (self.alpha / COLOR_RANGE) as f64;

        Color { r, g, b, a }
    }
}

impl Colour
{
    pub const WHITE:          Colour = Colour { red: 255, green: 255, blue: 255, alpha: 255 };
    pub const RED:            Colour = Colour { red: 255, green: 0, blue: 0, alpha: 255 };
    pub const GREEN:          Colour = Colour { red: 0, green: 255, blue: 0, alpha: 255 };
    pub const BLUE:           Colour = Colour { red: 0, green: 0, blue: 255, alpha: 255 };
    pub const BLACK:          Colour = Colour { red: 0, green: 0, blue: 0, alpha: 255 };
    pub const CORNFLOWERBLUE: Colour = Colour { red: 100, green: 149, blue: 237, alpha: 255 };
}