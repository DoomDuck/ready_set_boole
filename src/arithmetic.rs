pub fn adder(a: u32, b: u32) -> u32 {
    let mut result = 0;
    let mut c = 0;
    for i in 0..u32::BITS {
        let a = (a >> i) & 1;
        let b = (b >> i) & 1;
        result |= (a ^ b ^ c) << i;
        c = (a & b) | (b & c) | (a & c);
    }
    result
}

pub fn subber(a: u32, b: u32) -> u32 {
    !adder(!a, b)
}

pub fn multiplier(mut a: u32, b: u32) -> u32 {
    let mut result = 0;
    for i in 0..u32::BITS {
        if a & 1 == 1 {
            result = adder(result, b << i)
        }
        a >>= 1;
    }
    result
}

pub fn gray_code(a: u32) -> u32 {
    a ^ (a >> 1)
}

#[cfg(test)]
mod tests {
    #[test]
    fn adder() {
        // Some simple additions
        for a in 0..u8::MAX as u32 {
            for b in 0..u8::MAX as u32 {
                assert_eq!(super::adder(a, b), a.wrapping_add(b));
            }
        }
        // Edge cases
        assert_eq!(super::adder(u32::MAX, 1), 0);
    }

    #[test]
    fn subber() {
        // Some simple substractions
        for a in 0..u8::MAX as u32 {
            for b in 0..u8::MAX as u32 {
                assert_eq!(super::subber(a, b), a.wrapping_sub(b));
            }
        }
        // Edge cases are handled above
    }

    #[test]
    fn mutiplier() {
        // Som.e simple multiplication
        for a in 0..u8::MAX as u32 {
            for b in 0..u8::MAX as u32 {
                assert_eq!(super::multiplier(a, b), a.wrapping_mul(b));
            }
        }
        // Edge cases
        assert_eq!(
            super::multiplier(u32::MAX, u32::MAX),
            u32::MAX.wrapping_mul(u32::MAX)
        );
    }

    #[test]
    fn gray_code() {
        // Some simple multiplication
        for a in 0..u8::MAX as u32 {
            let ga = super::gray_code(a);
            let gb = super::gray_code(a + 1);
            let diff = ga ^ gb;
            assert_eq!(diff.count_ones(),  1, "{ga:b} {gb:b}");
        }

        // Examples from subject
        assert_eq!(super::gray_code(0), 0);
        assert_eq!(super::gray_code(1), 1);
        assert_eq!(super::gray_code(2), 3);
        assert_eq!(super::gray_code(3), 2);
        assert_eq!(super::gray_code(4), 6);
        assert_eq!(super::gray_code(5), 7);
        assert_eq!(super::gray_code(6), 5);
        assert_eq!(super::gray_code(7), 4);
        assert_eq!(super::gray_code(8), 12);
    }
}
