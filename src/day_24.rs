use regex::Regex;
use rug::Rational;
use std::{
    fs,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone)]
struct PointVel {
    px: Rational,
    py: Rational,
    pz: Rational,
    vx: Rational,
    vy: Rational,
    vz: Rational,
}

#[derive(Debug, Clone, PartialEq)]
struct Vect {
    x: Rational,
    y: Rational,
    z: Rational,
}

impl<'a> Add<&'a Vect> for &'a Vect {
    type Output = Vect;
    fn add(self, rhs: Self) -> Self::Output {
        Vect {
            x: self.clone().x + rhs.clone().x,
            y: self.clone().y + rhs.clone().y,
            z: self.clone().z + rhs.clone().z,
        }
    }
}

impl<'a> Sub<&'a Vect> for &'a Vect {
    type Output = Vect;
    fn sub(self, rhs: Self) -> Self::Output {
        Vect {
            x: self.clone().x - rhs.clone().x,
            y: self.clone().y - rhs.clone().y,
            z: self.clone().z - rhs.clone().z,
        }
    }
}

impl<'a> Mul<&Rational> for &'a Vect {
    type Output = Vect;
    fn mul(self, a: &Rational) -> Self::Output {
        Vect {
            x: a * self.clone().x,
            y: a * self.clone().y,
            z: a * self.clone().z,
        }
    }
}

impl<'a> Div<&Rational> for &'a Vect {
    type Output = Vect;
    fn div(self, a: &Rational) -> Self::Output {
        Vect {
            x: self.clone().x / a,
            y: self.clone().y / a,
            z: self.clone().z / a,
        }
    }
}

impl<'a> PointVel {
    fn point(&'a self) -> Vect {
        Vect {
            x: self.clone().px,
            y: self.clone().py,
            z: self.clone().pz,
        }
    }

    fn velocity(&'a self) -> Vect {
        Vect {
            x: self.clone().vx,
            y: self.clone().vy,
            z: self.clone().vz,
        }
    }

    fn at_time(self, t: Rational) -> Vect {
        &self.point() + &(&self.velocity() * &t)
    }
}

#[derive(Debug)]
struct QEq {
    /// a quadratic equation of the form axy xy + ax x + ay y + a0 = 0
    axy: Rational,
    ax: Rational,
    ay: Rational,
    a0: Rational,
}

fn det(p1: &Vect, p2: &Vect, p3: &Vect) -> Rational {
    p1.clone().x * p2.clone().y * p3.clone().z
        + p1.clone().y * p2.clone().z * p3.clone().x
        + p1.clone().z * p2.clone().x * p3.clone().y
        - p1.clone().z * p2.clone().y * p3.clone().x
        - p1.clone().y * p2.clone().x * p3.clone().z
        - p1.clone().x * p2.clone().z * p3.clone().y
}

fn sqrt(q: &Rational) -> Rational {
    let (num, den) = (q.numer(), q.denom());
    let ret = Rational::from((num.clone().sqrt(), den.clone().sqrt()));
    assert_eq!(ret.clone() * ret.clone(), *q);
    ret
}

fn solve_quadratic(a: Rational, b: Rational, c: Rational) -> Vec<Rational> {
    assert!((a != 0.0) | (b != 0.0));
    let mut ret: Vec<Rational> = Vec::new();
    if a == 0.0 {
        ret.push(-c / b);
    } else {
        let d = b.clone() * b.clone() - Rational::from((4, 1)) * a.clone() * c.clone();
        if d > 0.0 {
            ret.push((-b.clone() + sqrt(&d)) / (Rational::from((2, 1)) * a.clone()));
            ret.push((-b.clone() - sqrt(&d)) / (Rational::from((2, 1)) * a.clone()));
        }
    }
    ret
}

impl QEq {
    fn is_zero(&self) -> bool {
        (self.axy == 0.0) & (self.ax == 0.0) & (self.ay == 0.0) & (self.a0 == 0.0)
    }
    fn solve_with(self, other: &QEq) -> Vec<(Rational, Rational)> {
        // get a linear eq. first of the form lx x + ly y + l0 = 0
        // and then use it to find solutions of both quad eqs.
        assert!(!self.is_zero());
        assert!(!other.is_zero());
        let lx: Rational;
        let ly: Rational;
        let l0: Rational;
        let qeq: &QEq;
        println!("  self: {:?}", self);
        if self.axy == 0.0 {
            (lx, ly, l0, qeq) = (self.ax, self.ay, self.a0, other);
        } else {
            (lx, ly, l0, qeq) = (
                other.ax.clone() - self.ax.clone() * other.axy.clone() / self.axy.clone(),
                other.ay.clone() - self.ay.clone() * other.axy.clone() / self.axy.clone(),
                other.a0.clone() - self.a0.clone() * other.axy.clone() / self.axy.clone(),
                &self,
            );
        }
        assert!((lx != 0.0) | (ly != 0.0) | (l0 != 0.0));
        if lx == 0.0 {
            let mgf = -l0 / ly;
            return vec![(
                (qeq.a0.clone() + qeq.ay.clone() * mgf.clone())
                    / (qeq.ax.clone() + qeq.axy.clone() * mgf.clone()),
                mgf.clone(),
            )];
        }
        let mfe = -ly / lx.clone();
        let mge = -l0 / lx.clone();
        let sols = solve_quadratic(
            qeq.axy.clone() * mfe.clone(),
            qeq.axy.clone() * mge.clone() + qeq.ax.clone() * mfe.clone() + qeq.ay.clone(),
            qeq.ax.clone() * mge.clone() + qeq.a0.clone(),
        );
        return sols
            .iter()
            .map(|y| (mfe.clone() * y + mge.clone(), y.clone()))
            .collect();
    }
    fn from_three_pvs(pv1: &PointVel, pv2: &PointVel, pv3: &PointVel) -> QEq {
        let axy = det(&pv2.velocity(), &pv1.velocity(), &pv3.velocity());
        let ax = det(&pv1.velocity(), &pv3.point(), &pv3.velocity())
            + det(&pv2.point(), &pv1.velocity(), &pv3.velocity());
        let ay = det(&pv2.velocity(), &pv1.point(), &pv3.velocity())
            - det(&pv2.velocity(), &pv3.point(), &pv3.velocity());
        let a0 = det(&pv1.point(), &pv3.point(), &pv3.velocity())
            + det(&pv2.point(), &pv1.point(), &pv3.velocity())
            - det(&pv2.point(), &pv3.point(), &pv3.velocity());
        QEq { axy, ax, ay, a0 }
    }
}

fn parse_input(input: Vec<&str>) -> Vec<PointVel> {
    let pvre = Regex::new(r"(-?\d+), +(-?\d+), +(-?\d+) @ +(-?\d+), +(-?\d+), +(-?\d+)").unwrap();
    input
        .iter()
        .map(|l| {
            let m = pvre.captures(l).unwrap();
            PointVel {
                px: m.get(1).unwrap().as_str().parse().unwrap(),
                py: m.get(2).unwrap().as_str().parse().unwrap(),
                pz: m.get(3).unwrap().as_str().parse().unwrap(),
                vx: m.get(4).unwrap().as_str().parse().unwrap(),
                vy: m.get(5).unwrap().as_str().parse().unwrap(),
                vz: m.get(6).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect()
}

fn cross_in_future(pv1: &PointVel, pv2: &PointVel) -> Option<(Rational, Rational)> {
    let d: Rational = pv1.vx.clone() * pv2.vy.clone() - pv1.vy.clone() * pv2.vx.clone();
    if d == 0.0 {
        return None;
    }
    let p1p2 = (
        pv1.px.clone() - pv2.px.clone(),
        pv1.py.clone() - pv2.py.clone(),
    );
    let a: Rational =
        (-pv2.vy.clone() * p1p2.0.clone() + pv2.vx.clone() * p1p2.1.clone()) / d.clone();
    let b: Rational =
        (-pv1.vy.clone() * p1p2.0.clone() + pv1.vx.clone() * p1p2.1.clone()) / d.clone();
    if (a < 0.0) | (b < 0.0) {
        return None;
    }
    Some((
        pv1.px.clone() + a.clone() * pv1.vx.clone(),
        pv1.py.clone() + a.clone() * pv1.vy.clone(),
    ))
}

fn prob1(
    input: Vec<&str>,
    limits_x: (&Rational, &Rational),
    limits_y: (&Rational, &Rational),
) -> u64 {
    let pvs = parse_input(input);
    let mut ret = 0;
    for (i, pv1) in pvs.iter().enumerate() {
        for pv2 in pvs.get(i + 1..).unwrap().iter() {
            let cif = cross_in_future(pv1, pv2);
            if is_inside(cif, limits_x, limits_y) {
                ret += 1;
            }
        }
    }
    ret
}

fn get_magic_point(input: Vec<&str>) -> Vec<PointVel> {
    let pvs = parse_input(input);
    assert!(pvs.len() >= 4);

    let mut ret: Vec<PointVel> = Vec::new();
    for s in 0..4 {
        // let qeq1 = QEq::from_three_pvs(&pvs[0], &pvs[1], &pvs[2]);
        // let qeq2 = QEq::from_three_pvs(&pvs[0], &pvs[1], &pvs[3]);
        let qeq1 = QEq::from_three_pvs(&pvs[s], &pvs[s + 1], &pvs[s + 2]);
        let qeq2 = QEq::from_three_pvs(&pvs[s], &pvs[s + 1], &pvs[s + 3]);
        let l0l1 = qeq1.solve_with(&qeq2);
        for (t0, t1) in l0l1.iter() {
            let p0 = pvs[s].clone().at_time(t0.clone());
            let p1 = pvs[s + 1].clone().at_time(t1.clone());
            let t1t0inv = (t1.clone() - t0.clone()).recip();
            let vel = &(&p1 - &p0) * &t1t0inv;
            let pos = &p0 - &(&vel * &t0);
            ret.push(PointVel {
                px: pos.x,
                py: pos.y,
                pz: pos.z,
                vx: vel.x,
                vy: vel.y,
                vz: vel.z,
            })
        }
    }
    ret
}

fn prob2(input: Vec<&str>) -> Vec<Rational> {
    let mps = get_magic_point(input);
    for magic_point in mps.iter() {
        println!(
            "  -> mp: {:?}, sum: {}",
            magic_point,
            magic_point.clone().px + magic_point.clone().py + magic_point.clone().pz
        );
    }
    mps.iter()
        .map(|p| p.clone().px + p.clone().py + p.clone().pz)
        .collect()
}

fn is_inside(
    xy: Option<(Rational, Rational)>,
    limits_x: (&Rational, &Rational),
    limits_y: (&Rational, &Rational),
) -> bool {
    if xy.is_none() {
        return false;
    }
    let xy = xy.unwrap();
    (limits_x.0 <= &xy.0) & (&xy.0 <= limits_x.1) & (limits_y.0 <= &xy.1) & (&xy.1 <= limits_y.1)
}

pub fn main() {
    let input = fs::read_to_string("day_24_input").expect("no input file");
    let input: Vec<&str> = input.trim().split("\n").collect();
    let lim0 = Rational::from((200000000000000i64, 1));
    let lim1 = Rational::from((400000000000000i64, 1));
    println!(
        "prob1: {}",
        prob1(input.clone(), (&lim0, &lim1), (&lim0, &lim1),)
    );
    println!("prob2: {:?}", prob2(input));
}

#[cfg(test)]
mod tests {
    use rug::Rational;

    use crate::day_24::{parse_input, prob1, prob2, sqrt, Vect};

    fn example() -> Vec<&'static str> {
        vec![
            "19, 13, 30 @ -2,  1, -2",
            "18, 19, 22 @ -1, -1, -2",
            "20, 25, 34 @ -2, -2, -4",
            "12, 31, 28 @ -1, -2, -1",
            "20, 19, 15 @  1, -5, -3",
        ]
    }

    #[test]
    fn test_prob1() {
        let crosses = prob1(
            example(),
            (&Rational::from((10, 1)), &Rational::from((20, 1))),
            (&Rational::from((10, 1)), &Rational::from((20, 1))),
        );
        assert_eq!(crosses, 2);
    }

    #[test]
    fn test_parse_input() {
        let pvs = parse_input(example());
        assert_eq!(pvs.len(), 5);
        assert_eq!(
            pvs[0].point(),
            Vect {
                x: Rational::from((19, 1)),
                y: Rational::from((13, 1)),
                z: Rational::from((30, 1))
            }
        );
    }

    #[test]
    fn test_prob2() {
        let ret = prob2(example());
        assert_eq!(ret, vec![47.0]);
    }

    #[test]
    fn test_sqrt() {
        let a = Rational::from((974327, 29771));
        let aa = a.clone() * a.clone();
        assert_eq!(a, sqrt(&aa));
    }
}
