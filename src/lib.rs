#[inline]
pub fn euler1_simple(input: i64) -> i64 {
    let mut sum: i64 = 0;
    for i in 1..input {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i as i64;
        }
    }
    sum
}

#[inline]
pub fn euler1_par(input: i64) -> i64 {
    use std::thread;
    use std::thread::JoinHandle;

    let threads = thread::available_parallelism().unwrap().get() as f64;
    let input = input as f64;
    let mut handles: Vec<JoinHandle<i64>> = vec![];

    for t in 1..=(threads as i32) {
        let t = t as f64;
        let upper_bound = (input * (t / threads)) as i32;
        let lower_bound = (input * (t - 1f64) / threads) as i32;

        handles.push(thread::spawn(move || {
            let mut sum: i64 = 0;
            for i in lower_bound..upper_bound {
                if i % 3 == 0 || i % 5 == 0 {
                    sum += i as i64;
                }
            }
            sum
        }));
    }

    let mut sum = 0;

    for h in handles {
        sum += h.join().unwrap();
    }
    sum
}

#[inline]
pub fn euler1_series(input: i64) -> i64 {
    let val = input - 1;
    let n_3 = val / 3;
    let n_5 = val / 5;
    let n_15 = val / 15;

    let sum_three = 3 * n_3 * (n_3 + 1) / 2;
    let sum_five = 5 * n_5 * (n_5 + 1) / 2;
    let sum_fifteen = 15 * n_15 * (n_15 + 1) / 2;

    sum_three + sum_five - sum_fifteen
}
