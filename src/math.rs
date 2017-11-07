pub fn next_pow2_u32(value: u32) -> u32 {
    2u32.pow(((value as f32).log2().ceil()) as u32)
}