use num::bigint::BigInt;
use num::{Signed, ToPrimitive};

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a > 0 && b >= 0 {
        let (mut a, mut b) = (a, b);
        let (mut taa, mut tab): (i64, i64) = (1, 0);
        let (mut tba, mut tbb): (i64, i64) = (0, 1);
        while b > 0 {
            let q = a / b;
            let r = a % b;
            // a = q * b + r = taa * A + tab * B, b = tba * A + tbb * B
            a = b;
            b = r;
            (taa, tba) = (tba, taa - q * tba);
            (tab, tbb) = (tbb, tab - q * tbb);
        }
        return (a, taa, tab);
    } else {
        let (m, ta, tb) = extended_gcd(a.abs(), b.abs());
        return (m, ta * a.signum(), tb * b.signum());
    }
}
fn crt_and_lcm(r0: i64, r1: i64, m0: u64, m1: u64) -> Option<(u64, u64)> {
    let (gcd, t0, t1) = extended_gcd(m0 as i64, m1 as i64);
    if r0 % gcd != 0 || r1 % gcd != 0 {
        return None;
    }
    // g = t0 * m0 + t1 * m1,
    // x = t1 * (r0/g) * m0 + t1 * (r1/g) * m1
    // let lcm = (m0 * m1).abs() / gcd;
    // let result = t0 * (r1 / gcd) * m0 + t1 * (r0 / gcd) * m1;
    // let result = (result % lcm + lcm) % lcm;
    let lcm = BigInt::from(m0 * m1) / gcd;
    let result = BigInt::from(t0) * BigInt::from(r1 / gcd) * BigInt::from(m0)
        + BigInt::from(t1) * BigInt::from(r0 / gcd) * BigInt::from(m1);
    let result = to_positive_congruence(result, lcm.clone());
    Some((result.to_u64().unwrap(), lcm.to_u64().unwrap()))
}

fn to_positive_congruence(c: BigInt, m: BigInt) -> BigInt {
    let m: &BigInt = &m.abs();
    if c.is_positive() {
        c % m
    } else {
        ((c % m) + m) % m
    }
}

/// if the solution is 0, returns the LCM
pub fn chinese_reminder_theorem(reminders: &[i64], moduli: &[u64]) -> Option<u64> {
    let (mut result, mut lcm) = (0u64, 1u64);
    for (&r0, &m0) in reminders.iter().zip(moduli.iter()) {
        let (nextr, nextlcm) = crt_and_lcm(result as i64, r0, lcm, m0)?;
        result = nextr;
        lcm = nextlcm;
    }
    if result == 0 {
        Some(lcm)
    } else {
        Some(result as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::{chinese_reminder_theorem, extended_gcd};

    #[test]
    fn test_egcd() {
        let (m, ta, tb) = extended_gcd(3, 5);
        assert_eq!(m, 1);
        assert_eq!(ta * 3 + tb * 5, 1);

        let (m, ta, tb) = extended_gcd(33, -9);
        assert_eq!(m, 3);
        assert_eq!(ta * 33 - tb * 9, 3);

        let (m, ta, tb) = extended_gcd(-32, 24);
        assert_eq!(m, 8);
        assert_eq!(-ta * 32 + tb * 24, 8);

        let (m, ta, tb) = extended_gcd(-32, -24);
        assert_eq!(m, 8);
        assert_eq!(-ta * 32 - tb * 24, 8);
    }
    #[test]
    fn test_crt() {
        assert_eq!(chinese_reminder_theorem(&[3, 5], &[7, 11]), Some(38));
        assert!(
            chinese_reminder_theorem(&[3889, 4048, 3072, 3767], &[3889, 3979, 2511, 3767])
                .is_some()
        );
        assert_eq!(
            chinese_reminder_theorem(&[3889, 4048, 3072, 3767], &[3889, 3979, 2511, 3767]).unwrap(),
            231
        );
    }
}
