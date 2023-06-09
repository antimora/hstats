use hstats::Hstats;
use rand::SeedableRng;
use rand_distr::{Distribution, Normal};
use rayon::{prelude::ParallelIterator, slice::ParallelSlice};

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
    // Create a random number generator with a fixed seed for reproducibility
    let mut rng = rand::rngs::StdRng::seed_from_u64(SEED);

    // Create a normal distribution with mean 2.0 and standard deviation 3.0
    let normal = Normal::<T>::new(MEAN, STD_DEV).unwrap();

    // Generate some random data
    let random_data: Vec<T> = (0..NUM_SAMPLES).map(|_x| normal.sample(&mut rng)).collect();

    println!("Number of random samples: {}", random_data.len());
    println!("Number of bins: {}", NUM_BINS);
    println!("Start: {}", START);
    println!("End: {}", END);

    // Get the number of threads
    let thread_count = rayon::current_num_threads() * 2;
    println!("Thread count: {}", thread_count);

    let chunk_size = random_data.len() / thread_count;

    println!("Chunk size: {}", chunk_size);

    // Update the stats for each chunk in parallel
    let hstats_list: Vec<Hstats<T>> = random_data
        .par_chunks(chunk_size)
        .map(|chunk| {
            let mut hstats = Hstats::new(START, END, NUM_BINS);
            chunk.iter().for_each(|v| hstats.add(*v));
            hstats
        })
        .collect();

    println!("Number of hstats to merge: {}", hstats_list.len());

    // Merge the stats from each chunk
    let merged = hstats_list
        .into_iter()
        .reduce(|hstats1, hstats2| hstats1.merge(&hstats2))
        .unwrap();

    // Print histogram graph
    println!("{}", merged.with_precision(2));
}
