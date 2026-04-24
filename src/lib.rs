use std::time::Instant;
use rayon::prelude::*;
use std::hint::black_box;
use rayon::ThreadPoolBuilder;


#[derive(Clone)]
pub struct Event {
    x: u16,
    y: u16,
    time: i64, //units are 1.5625ns (tpx manual)
    intens: u16,
}

pub fn benchmark() {
    for i in 1..=8{
            for _x in 0..5{
                let start: Instant = Instant::now();
                let mut out_vec: Vec<Event> = (0..360000000)
                    .into_par_iter()
                    .map(|_| Event {
                        x: fastrand::u16(..),
                        y: fastrand::u16(..),
                        time: fastrand::i64(..),
                        intens: fastrand::u16(..),
                    })
                    .collect();
                let duration = start.elapsed();
                println!("    Generating hits took: {:.3?}", duration);

                let n_threads = i;
                let pool = ThreadPoolBuilder::new()
                    .num_threads(n_threads)
                    .build()
                    .unwrap();
                
                let start: Instant = Instant::now();
                pool.install(|| {
                    let _out_vec2: Vec<Event> = black_box(out_vec.par_iter().cloned().collect());
                });
                let duration = start.elapsed();
                println!("    Cloning hits took: {:.3?} n_threads: {n_threads}", duration);

                
                let start: Instant = Instant::now();
                pool.install(|| {
                    out_vec.par_sort_unstable_by(|a, b| a.time.cmp(&b.time));
                });
                let duration = start.elapsed();
                println!("    Sorting hits took: {:.3?} n_threads: {n_threads}", duration);
                
                
                println!("{}",out_vec[0].x+ out_vec[0].y+ out_vec[0].intens);
            }
        }

        // let start: Instant = Instant::now();
        // let out_vec2 = black_box(out_vec.clone());

        println!("done!");
}