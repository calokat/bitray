#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Default for Interval {
    fn default() -> Self {
        return Self {
            min: f32::MAX,
            max: f32::MIN,
        };
    }
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
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

    pub fn expand(&mut self, delta: f32) {
        let padding = delta / 2.0;
        self.min -= padding;
        self.max += padding;
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        let t_min = f32::max(self.min, other.min);
        let t_max = f32::min(self.max, other.max);
        return t_min < t_max;
    }

    pub fn to_contain(&self, other: &Self) -> Self {
        Self {
            min: f32::min(self.min, other.min),
            max: f32::max(self.max, other.max),
        }
    }
}
