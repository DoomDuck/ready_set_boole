
/// Create a Z-order curve mapping
pub fn map(x: u16, y: u16) -> f64 {
    let packed = (x as u64) | ((y as u64) << 32);

    let first = (packed | (packed << 8)) & 0x00FF00FF00FF00FF;
    let second = (first | (first << 4)) & 0x0F0F0F0F0F0F0F0F;
    let third = (second | (second << 2)) & 0x3333333333333333;
    let fourth = (third | (third << 1)) & 0x5555555555555555;

    let x = fourth;
    let y = fourth >> 31;
    ((x | y) as u32 as f64) / (u32::MAX as f64)
}

pub fn reverse_map(n: f64) -> (u16, u16) {
    let wide_idx = (n * u32::MAX as f64) as u64;
    let packed = (wide_idx & 0x55555555) | ((wide_idx & 0xAAAAAAAA) << 31);

    let first = (packed | (packed >> 1)) & 0x3333333333333333;
    let second = (first | (first >> 2)) & 0x0F0F0F0F0F0F0F0F;
    let third = (second | (second >> 4)) & 0x00FF00FF00FF00FF;
    let fourth = third | (third >> 8);

    let x = fourth as u16;
    let y = (fourth >> 32) as u16;
    (x, y)
}

#[cfg(test)]
mod tests {
    #[test]
    fn reversible() {
        for x in (0..=u16::MAX).step_by(132) {
            for y in (0..=u16::MAX).step_by(45) {
                let n = super::map(x, y);
                assert_eq!((x, y), super::reverse_map(n));
            }
        }
    }
}
