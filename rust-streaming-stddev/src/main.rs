use std::thread;
use std::time::Duration;

fn main() {
    // println!("Hello, world!");
    // stdev(vec![0, 1, 2]);
    // threads2();
    // threads3();
    let input = vec![1., 2.];
    println!("input: {:?}", input);
    let ans = stddev(input);
    println!("stddev: {}", ans);
}

fn stdev(arr: Vec<i32>) {
    let s: i32 = arr.iter().sum();
    println!("{}", s);
}

fn threads() {
    let a = |x| {
        println!("{}", x * 2);
    };
    a(1);
    a(2);
}

fn threads2() {
    const THREADS: u32 = 30;

    let mut children = vec![];

    for i in 1..=THREADS {
        let handler = thread::spawn(move || {
            println!("this is thread {}", i);
            thread::sleep(Duration::from_secs(5));
        });
        children.push(handler);
    }

    for handler in children {
        let _ = handler.join();
    }
}

fn threads3() {
    const THREADS: u32 = 100;

    let mut handles = vec![];
    for i in 1..=THREADS {
        let handle = thread::spawn(move || {
            const DUR: u64 = 2;
            println!("in thread {}", i);
            thread::sleep(Duration::from_secs(DUR));
            println!("done sleeping for {} secs", DUR);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn variance(arr: Vec<f64>) -> f64 {
    let sum: f64 = arr.iter().sum();
    let n: f64 = arr.len() as f64;

    let mean = sum / n;

    let mut mean_sq_diffs: Vec<f64> = vec![];
    for el in arr {
        let diff_sq = (el - mean).powf(2.0);
        mean_sq_diffs.push(diff_sq);
    }

    let var: f64 = mean_sq_diffs.iter().sum::<f64>() / n;
    return var;
}

fn stddev(arr: Vec<f64>) -> f64 {
    variance(arr).sqrt()
}
