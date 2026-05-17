pub struct Interval
{
    pub min: f64,
    pub max: f64,
}

impl Interval
{
    pub const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    pub const UNIVERSE: Interval = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    pub fn default() -> Interval {
        Interval::EMPTY
    }

    pub const fn new(min: f64, max: f64) -> Interval{
        Interval{
            min : min,
            max : max,
        }
    }

    pub fn size(&self) -> f64{
        return self.max - self.min;
    }

    pub fn contains(&self, x: f64) -> bool{
        return self.min <= x && x <= self.max;
    }

    pub fn surrounds(&self, x: f64) -> bool{
        return self.min < x && x < self.max;
    }

    pub fn clamp(&self, x: f64) -> f64{
        if x < self.min
        {
            return self.min;
        }
        if x > self.max
        {
            return self.max;
        }
        return x;
    }
}