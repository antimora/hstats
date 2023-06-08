use hstats::Hstats;
use rand::SeedableRng;
use rand_distr::{Distribution, Normal};

// Define some constants
const MEAN: T = 2.0;
const STD_DEV: T = 3.0;
const SEED: u64 = 42;
const NUM_SAMPLES: usize = 50_000_000;
const NUM_BINS: usize = 30;
const START: T = -8.0;
const END: T = 10.0;

type T = f64;

pub fn main() {
    let mut hstats = Hstats::new(START, END, NUM_BINS);

    // Create a random number generator with a fixed seed for reproducibility
    let mut rng = rand::rngs::StdRng::seed_from_u64(SEED);

    // Create a normal distribution with mean 2.0 and standard deviation 3.0
    let normal = Normal::<T>::new(MEAN, STD_DEV).unwrap();

    // Generate some random data
    let random_data: Vec<T> = (0..NUM_SAMPLES).map(|_x| normal.sample(&mut rng)).collect();

    // Update the stats
    random_data.iter().for_each(|v| hstats.add(*v));

    // Print histogram graph

    println!("{}", hstats.with_precision(2));
}
