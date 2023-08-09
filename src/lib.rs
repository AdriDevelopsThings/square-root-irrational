#[cfg(test)]
mod test;

/// Get a list of prim_factors of `n`
/// Each element does look like this:
/// ```ignore
/// let (factor, count) = element;
/// ```
/// so `factor ^ count`
pub(crate) fn prim_factors(n: u32) -> Vec<(u32, u8)> {
    let mut factors: Vec<(u32, u8)> = Vec::new();
    let mut k = n;
    for i in 2..n {
        let mut c : u8= 0;
        while k % i == 0 {
            c += 1;
            k /= i;
        }
        if c > 0 {
            factors.push((i, c));
        }
        if k == 1 {
            break;
        }
    }
    factors
}

/// Check if the square root of the number `n` is rational (`true`) or irrational (`false`)
pub fn is_square_root_rational(n: u32) -> bool {
    let prim_factors = prim_factors(n);
    if prim_factors.is_empty() && !(n == 0 || n == 1) {
        return false;
    }
    for (_, count) in prim_factors {
        if count % 2 != 0 {
            return false;
        }
    }
    true
}