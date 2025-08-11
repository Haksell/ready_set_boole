/// Implementation of https://en.wikipedia.org/wiki/Z-order_curve

/// Interleaves a 16-bit number with zeros
/// http://graphics.stanford.edu/%7Eseander/bithacks.html#InterleaveBMN
#[inline]
fn interleave(n: u16) -> u32 {
    let mut n = n as u32;
    // 0000000000000000abcdefghijklmnop
    n = (n | n << 8) & 0x00ff00ff;
    // 00000000abcdefgh00000000ijklmnop
    n = (n | n << 4) & 0x0f0f0f0f;
    // 0000abcd0000efgh0000ijkl0000mnop
    n = (n | n << 2) & 0x33333333;
    // 00ab00cd00ef00gh00ij00kl00mn00op
    n = (n | n << 1) & 0x55555555;
    // 0a0b0c0d0e0f0g0h0i0j0k0l0m0n0o0p
    n
}

pub fn map(x: u16, y: u16) -> f64 {
    let z = interleave(y) << 1 | interleave(x);
    z as f64 / u32::MAX as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_close(x: f64, y: f64) -> bool {
        (x - y).abs() < 1e-5
    }

    #[test]
    fn test_interleave() {
        assert_eq!(interleave(0), 0);
        assert_eq!(interleave(0b101010), 0b010001000100);
        assert_eq!(interleave(0xffff), 0x55555555);
    }

    #[test]
    fn test_map() {
        assert_eq!(map(0, 0), 0.);
        assert!(is_close(map(u16::MAX, 0), 1. / 3.)); // 01010101... is 1/3 of u32::MAX
        assert!(is_close(map(0, u16::MAX), 2. / 3.)); // 10101010... is 2/3 of u32::MAX
        assert!(is_close(map(0x7fff, 0x7fff), 0.25)); // bottom-right of top-left corner
        assert!(is_close(map(0x8000, 0x0000), 0.25)); // top-left of top-right corner
        assert_eq!(map(u16::MAX, u16::MAX), 1.);
    }
}
