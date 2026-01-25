pub struct Rng {
    state: [u32; 4],
}

fn u64_to_unit_f64(x: u64) -> f64 {
    (x >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
}

impl Rng {
    pub fn new() -> Self {
        // stolen from rand_core/le.rs - read_u64_into()
        // for (out, chunk) in state.iter_mut().zip(seed.chunks_exact(4)) {
        //     *out = u32::from_le_bytes(chunk.try_into().unwrap());
        // }
        Self {
            state: [101058054, 1465341783, 202116108, 572662306],
        }
    }

    pub fn f32(&mut self) -> f32 {
        let result_starstar = self.state[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);

        let t = self.state[1] << 9;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;

        self.state[3] = self.state[3].rotate_left(11);

        return crate::fast_u32_to_f32(result_starstar);
    }
}
