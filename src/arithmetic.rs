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
        for a in 0..u8::MAX as u32 {
            for b in 0..u8::MAX as u32 {
                assert_eq!(super::adder(a, b), a.wrapping_add(b));
            }
        }
    }

    #[test]
    fn subber() {
        for a in 0..u8::MAX as u32 {
            for b in 0..u8::MAX as u32 {
                assert_eq!(super::subber(a, b), a.wrapping_sub(b));
            }
        }
    }

    #[test]
    fn mutiplier() {
        for a in 0..u8::MAX as u32 {
            for b in 0..u8::MAX as u32 {
                assert_eq!(super::multiplier(a, b), a.wrapping_mul(b));
            }
        }
    }
}
