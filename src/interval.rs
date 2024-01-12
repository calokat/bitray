pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Default for Interval {
    fn default() -> Self {
        return Self { min: f32::MAX, max: f32::MIN }
    }
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
        }
    }

    pub fn contains(&self, n: f32) -> bool {
        return self.min <= n && n <= self.max;
    }

    pub fn surrounds(&self, n: f32) -> bool {
        return self.min < n && n < self.max;
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn universe() -> Self {
        Self::new(f32::MIN, f32::MAX)
    }

}