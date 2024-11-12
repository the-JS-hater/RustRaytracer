pub static EMPTY: Interval = Interval {
    min: std::f64::INFINITY,
    max: -std::f64::INFINITY,
};

pub static UNIVERSE: Interval = Interval {
    min: -std::f64::INFINITY,
    max: std::f64::INFINITY,
};

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.max >= x && x >= self.min
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            min: -std::f64::INFINITY,
            max: std::f64::INFINITY,
        }
    }
}
