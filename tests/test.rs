use segment_tree::{RMaxQ, RMinQ, RSQ};

const KX: u32 = 123456789;
const KY: u32 = 362436069;
const KZ: u32 = 521288629;
const KW: u32 = 88675123;
pub struct Rand {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}
impl Rand {
    pub fn new(seed: u32) -> Rand {
        Rand {
            x: KX ^ seed,
            y: KY ^ seed,
            z: KZ,
            w: KW,
        }
    }
    pub fn rand(&mut self) -> u32 {
        let t = self.x ^ self.x.wrapping_shl(11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w ^= self.w.wrapping_shr(19) ^ t ^ t.wrapping_shr(8);
        return self.w;
    }
    pub fn shuffle<T>(&mut self, a: &mut [T]) {
        if a.len() == 0 {
            return;
        }
        let mut i = a.len() - 1;
        while i > 0 {
            let j = (self.rand() as usize) % (i + 1);
            a.swap(i, j);
            i -= 1;
        }
    }
    pub fn rand_range(&mut self, a: i32, b: i32) -> i32 {
        let m = (b - a + 1) as u32;
        return a + (self.rand() % m) as i32;
    }
    pub fn rand_float(&mut self) -> f64 {
        (self.rand() as f64) / (<u32>::max_value() as f64)
    }
}

#[test]
fn test_rsq_1() {
    let values = [1, 2, 3, 4, 5];
    let mut rsq = RSQ::build(&values).unwrap();

    assert_eq!(rsq.sum(0, 4).unwrap(), 15);
    assert_eq!(rsq.sum(1, 4).unwrap(), 14);
    assert_eq!(rsq.sum(1, 2).unwrap(), 5);
    assert_eq!(rsq.sum(1, 1).unwrap(), 2);

    rsq.upddate(2, 6).unwrap();
    assert_eq!(rsq.sum(0, 4).unwrap(), 18);
    assert_eq!(rsq.sum(2, 4).unwrap(), 15);
    assert_eq!(rsq.sum(3, 4).unwrap(), 9);

    rsq.upddate(0, 5).unwrap();
    assert_eq!(rsq.sum(0, 0).unwrap(), 5);
    assert_eq!(rsq.sum(0, 4).unwrap(), 22);
}

#[test]
fn test_rsq_2() {
    let values = [1, 2, 3, 4, 5];
    let mut rsq = RSQ::build(&values).unwrap();

    rsq.upddate(0, 2).unwrap();
    assert_eq!(rsq.sum(0, 4).unwrap(), 16);
    assert_eq!(rsq.sum(2, 4).unwrap(), 12);
    assert_eq!(rsq.sum(3, 4).unwrap(), 9);
}

#[test]
fn test_rsq_3() {
    let mut rng = Rand::new(0);
    let mut values = vec![];
    for _ in 0..10000 {
        values.push(rng.rand_range(0, 100));
    }
    let mut rsq = RSQ::build(&values).unwrap();
    for _ in 0..1000 {
        let l = rng.rand_range(0, 9998);
        let r = rng.rand_range(l, 9999);
        if l <= r {
            assert_eq!(
                rsq.sum(l as usize, r as usize).unwrap(),
                (&values[l as usize..r as usize + 1]).iter().sum()
            );
        }
    }

    for _ in 0..1000 {
        let pos = rng.rand_range(0, 9999);
        let value = rng.rand_range(0, 1000);
        rsq.upddate(pos as usize, value).unwrap();
        values[pos as usize] = value;
    }

    for _ in 0..1000 {
        let l = rng.rand_range(0, 9998);
        let r = rng.rand_range(l, 9999);
        if l <= r {
            assert_eq!(
                rsq.sum(l as usize, r as usize).unwrap(),
                (&values[l as usize..r as usize + 1]).iter().sum()
            );
        }
    }
}

#[test]
fn test_rsq_4() {
    let values = [1, 2, 3, 4, 5];
    let mut rmq = RMaxQ::build(&values).unwrap();

    assert_eq!(rmq.max(0, 4).unwrap(), 5);
    assert_eq!(rmq.max(1, 4).unwrap(), 5);
    assert_eq!(rmq.max(1, 2).unwrap(), 3);
    assert_eq!(rmq.max(1, 1).unwrap(), 2);

    rmq.upddate(2, 6).unwrap();
    assert_eq!(rmq.max(0, 4).unwrap(), 6);
    assert_eq!(rmq.max(2, 4).unwrap(), 6);
    assert_eq!(rmq.max(3, 4).unwrap(), 5);
}

#[test]
fn test_rsq_5() {
    let mut rng = Rand::new(0);
    let mut values = vec![];
    for _ in 0..10000 {
        values.push(rng.rand_range(0, 100000));
    }
    let mut rmq = RMaxQ::build(&values).unwrap();
    for _ in 0..1000 {
        let l = rng.rand_range(0, 9998);
        let r = rng.rand_range(l, 9999);
        if l <= r {
            assert_eq!(
                rmq.max(l as usize, r as usize).unwrap(),
                *(&values[l as usize..r as usize + 1]).iter().max().unwrap()
            );
        }
    }

    for _ in 0..1000 {
        let pos = rng.rand_range(0, 9999);
        let value = rng.rand_range(0, 100000);
        rmq.upddate(pos as usize, value).unwrap();
        values[pos as usize] = value;
    }

    for _ in 0..1000 {
        let l = rng.rand_range(0, 9998);
        let r = rng.rand_range(l, 9999);
        if l <= r {
            assert_eq!(
                rmq.max(l as usize, r as usize).unwrap(),
                *(&values[l as usize..r as usize + 1]).iter().max().unwrap()
            );
        }
    }
}

#[test]
fn test_rsq_6() {
    let values = [1, 2, 3, 4, 5];
    let mut rmq = RMinQ::build(&values).unwrap();

    assert_eq!(rmq.min(0, 4).unwrap(), 1);
    assert_eq!(rmq.min(1, 4).unwrap(), 2);
    assert_eq!(rmq.min(1, 2).unwrap(), 2);
    assert_eq!(rmq.min(1, 1).unwrap(), 2);

    rmq.upddate(2, 0).unwrap();
    assert_eq!(rmq.min(0, 4).unwrap(), 0);
    assert_eq!(rmq.min(2, 4).unwrap(), 0);
    assert_eq!(rmq.min(3, 4).unwrap(), 4);
}

#[test]
fn test_rsq_7() {
    let mut rng = Rand::new(0);
    let mut values = vec![];
    for _ in 0..10000 {
        values.push(rng.rand_range(0, 100000));
    }
    let mut rmq = RMinQ::build(&values).unwrap();
    for _ in 0..1000 {
        let l = rng.rand_range(0, 9998);
        let r = rng.rand_range(l, 9999);
        if l <= r {
            assert_eq!(
                rmq.min(l as usize, r as usize).unwrap(),
                *(&values[l as usize..r as usize + 1]).iter().min().unwrap()
            );
        }
    }

    for _ in 0..1000 {
        let pos = rng.rand_range(0, 9999);
        let value = rng.rand_range(0, 100000);
        rmq.upddate(pos as usize, value).unwrap();
        values[pos as usize] = value;
    }

    for _ in 0..1000 {
        let l = rng.rand_range(0, 9998);
        let r = rng.rand_range(l, 9999);
        if l <= r {
            assert_eq!(
                rmq.min(l as usize, r as usize).unwrap(),
                *(&values[l as usize..r as usize + 1]).iter().min().unwrap()
            );
        }
    }
}

#[test]
fn test_rsq_8() {
    let mut rng = Rand::new(0);
    for _ in 0..1000 {
        let mut values = vec![];
        let len = rng.rand_range(5, 1000) as usize;
        for _ in 0..len {
            values.push(rng.rand_range(0, 100000));
        }
        let rmq = RMinQ::build(&values).unwrap();
        for _ in 0..1000 {
            let l = rng.rand_range(0, len as i32 - 2);
            let r = rng.rand_range(l, len as i32 - 1);
            if l <= r {
                assert_eq!(
                    rmq.min(l as usize, r as usize).unwrap(),
                    *(&values[l as usize..r as usize + 1]).iter().min().unwrap()
                );
            }
        }
    }
}

#[test]
fn test_rsq_9() {
    let values = [1, 2, 3, 4, 5];
    let mut rsq = RSQ::build(&values).unwrap();

    assert_eq!(rsq.sum(4, 0).unwrap(), 15);
    assert_eq!(rsq.sum(4, 1).unwrap(), 14);
    assert_eq!(rsq.sum(2, 1).unwrap(), 5);
    assert_eq!(rsq.sum(1, 1).unwrap(), 2);

    rsq.upddate(2, 6).unwrap();
    assert_eq!(rsq.sum(4, 0).unwrap(), 18);
    assert_eq!(rsq.sum(4, 2).unwrap(), 15);
    assert_eq!(rsq.sum(4, 3).unwrap(), 9);

    rsq.upddate(0, 5).unwrap();
    assert_eq!(rsq.sum(0, 0).unwrap(), 5);
    assert_eq!(rsq.sum(4, 0).unwrap(), 22);
}

#[test]
#[should_panic]
fn test_rsq_10() {
    let values: Vec<i32> = vec![];
    let _rsq = RSQ::build(&values).unwrap();
}

#[test]
#[should_panic]
fn test_rsq_11() {
    let values = [1, 2, 3, 4, 5];
    let mut rsq = RSQ::build(&values).unwrap();

    rsq.upddate(5, 6).unwrap();
}
