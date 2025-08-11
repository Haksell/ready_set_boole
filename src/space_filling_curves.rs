/// Implementation of https://en.wikipedia.org/wiki/Z-order_curve

/// Interleaves a 16-bit number with zeros
/// http://graphics.stanford.edu/%7Eseander/bithacks.html#InterleaveBMN
fn set_even_bits(n: u16) -> u32 {
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

fn get_even_bits(mut n: u32) -> u16 {
    // 0a0b0c0d0e0f0g0h0i0j0k0l0m0n0o0p
    n = (n & 0x44444444) >> 1 | (n & 0x11111111);
    // 00ab00cd00ef00gh00ij00kl00mn00op
    n = (n & 0x30303030) >> 2 | (n & 0x03030303);
    // 0000abcd0000efgh0000ijkl0000mnop
    n = (n & 0x0f000f00) >> 4 | (n & 0x000f000f);
    // 00000000abcdefgh00000000ijklmnop
    n = (n & 0x00ff0000) >> 8 | (n & 0x000000ff);
    // 0000000000000000abcdefghijklmnop
    n as u16
}

pub fn map(x: u16, y: u16) -> f64 {
    let z = set_even_bits(x) | set_even_bits(y) << 1;
    z as f64 / u32::MAX as f64
}

pub fn reverse_map(z: f64) -> (u16, u16) {
    let z = (z * u32::MAX as f64) as u32;
    let x = get_even_bits(z);
    let y = get_even_bits(z >> 1);
    (x, y)
}

#[cfg(test)]
mod tests {
    use {super::*, rand::Rng};

    fn is_close(x: f64, y: f64) -> bool {
        (x - y).abs() < 1e-5
    }

    #[test]
    fn test_set_even_bits() {
        assert_eq!(set_even_bits(0), 0);
        assert_eq!(set_even_bits(0b101010), 0b010001000100);
        assert_eq!(set_even_bits(0xffff), 0x55555555);
    }

    #[test]
    fn test_get_even_bits() {
        assert_eq!(get_even_bits(0), 0);
        assert_eq!(get_even_bits(0b010001000100), 0b101010);
        assert_eq!(get_even_bits(0x55555555), 0xffff);
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

    #[test]
    fn test_reverse_map() {
        assert_eq!(reverse_map(-f64::INFINITY), (0, 0));
        assert_eq!(reverse_map(0.), (0, 0));
        assert_eq!(reverse_map(1. / 3.), (u16::MAX, 0)); // 01010101... is 1/3 of u32::MAX
        assert_eq!(reverse_map(2. / 3.), (0, u16::MAX)); // 10101010... is 2/3 of u32::MAX
        assert_eq!(
            reverse_map(0x3fffffff as f64 / u32::MAX as f64),
            (0x7fff, 0x7fff)
        ); // bottom-right of top-left corner
        assert_eq!(
            reverse_map(0x40000000 as f64 / u32::MAX as f64),
            (0x8000, 0x0000)
        ); // top-left of top-right corner
        assert_eq!(reverse_map(1.), (u16::MAX, u16::MAX));
        assert_eq!(reverse_map(f64::INFINITY), (u16::MAX, u16::MAX));
    }

    #[test]
    fn test_map_reverse_map() {
        let mut rng = rand::rng();
        for _ in 0..100 {
            let x_in = rng.random();
            let y_in = rng.random();
            let z = map(x_in, y_in);
            let (x_out, y_out) = reverse_map(z);
            assert_eq!((x_in, y_in), (x_out, y_out));
        }
    }

    #[test]
    fn test_reverse_map_map() {
        let mut rng = rand::rng();
        for _ in 0..100 {
            let z_in = rng.random_range(0.0..=1.0);
            let (x, y) = reverse_map(z_in);
            let z_out = map(x, y);
            assert!(is_close(z_in, z_out));
        }
    }
}
