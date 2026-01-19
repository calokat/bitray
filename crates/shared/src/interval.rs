use crate::Float;

#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub min: Float,
    pub max: Float,
}

impl Default for Interval {
    fn default() -> Self {
        return Self {
            min: Float::MAX,
            max: Float::MIN,
        };
    }
}

impl Interval {
    pub fn new(min: Float, max: Float) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, n: Float) -> bool {
        return self.min <= n && n <= self.max;
    }

    pub fn surrounds(&self, n: Float) -> bool {
        return self.min < n && n < self.max;
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn universe() -> Self {
        Self::new(Float::MIN, Float::MAX)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        let t_min = Float::max(self.min, other.min);
        let t_max = Float::min(self.max, other.max);
        return t_min < t_max;
    }

    pub fn to_contain(&self, other: &Self) -> Self {
        Self {
            min: Float::min(self.min, other.min),
            max: Float::max(self.max, other.max),
        }
    }

    pub fn clamp_min(&mut self, clamp: Float) {
        self.min = Float::max(self.min, clamp);
    }

    pub fn clamp_max(&mut self, clamp: Float) {
        self.max = Float::min(self.max, clamp);
    }

    pub fn stretch_min(&mut self, stretch: Float) {
        self.min = Float::min(self.min, stretch);
    }

    pub fn stretch_max(&mut self, stretch: Float) {
        self.max = Float::max(self.max, stretch);
    }
}
