use std::fmt::Debug;
use rand::Rng;

#[derive(Debug)]
pub struct Fenwick {
    items: Vec<i32>,
    total: i32,
}

impl Fenwick {
    /// Constructs an empty Fenwick tree of the specified size.
    pub fn of_size(size: usize) -> Self {
        Self {
            items: vec![0; size],
            total: 0
        }
    }

    /// Computes a partial sum in the interval [i, j) in `O(log n)`.
    pub fn sum(self: &Self, mut i: usize, mut j: usize) -> i32 {
        let mut sum = 0;

        while j > i {
            sum += self.items[j - 1];
            j &= j - 1;
        }
        while i > j {
            sum -= self.items[i - 1];
            i &= i - 1;
        }

        sum
    }

    /// Returns the value by index `i`.
    pub fn get(self: &Self, i: usize) -> i32 {
        self.sum(i, i + 1)
    }

    /// Sets the value by index `i` to `x`.
    pub fn set(self: &mut Self, i: usize, x: i32) {
        self.add(i, x - self.get(i));
    }

    /// Adds `x` to the value by index `i`.
    pub fn add(self: &mut Self, mut i: usize, x: i32) {
        let n = self.items.len();

        while i < n {
            self.items[i] += x;
            i |= i + 1;
        }

        self.total += x;
    }

    /// A weighted sampling of a single element.
    ///
    /// The algorithm uses a binary search of the smallest index `i`, such that the partial sum 
    /// in the interval [0, i] is greater than a generated uniform random value. Assuming that a
    /// uniform random value can be generated in `O(1)`, the complexity of the algorithm is `O(log^2 n)`.
    pub fn sample_one<R>(self: &Self, random: &mut R) -> u32
        where R: Rng + ?Sized
    {
        let x = random.gen_range(1..=self.total);

        let mut a = 0;
        let mut i = 0;
        let mut j = self.items.len() - 1;

        while i < j {
            let m = (i + j) / 2;
            let s = self.sum(i, m + 1);

            if a + s < x {
                i = m + 1;
                a += s;
            }
            else {
                j = m;
            }
        }

        j as u32
    }

    /// A weighted sampling of `m` elements with replacement.
    ///
    /// The algorithm sequentially samples `m` elements using the `sample_one` function. Hence,
    /// the complexity of the algorithm is `O(m log^2 n)`.
    pub fn sample_many<R>(self: &Self, m: usize, random: &mut R) -> Vec<u32>
        where R: Rng + ?Sized
    {
        (0..m).map(|_| self.sample_one(random)).collect()
    }
}
