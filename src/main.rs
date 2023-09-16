use std::time;

#[allow(unused)]
fn for_inclusive() {
    let mut _num: u16 = 0;
    for i in 0..=(u16::MAX - 1) {
        _num += i;
    }
}

#[allow(unused)]
fn for_exclusive() {
    let mut _num: u16 = 0;
    for i in 0..u16::MAX {
        _num += i;
    }
}

fn main() {
    println!("Running for 5 iterations");
    let start = time::Instant::now();
    for _ in 0..5 {
        // Uncomment the desired function to test
        //for_exclusive();
        for_inclusive();
    }
    let end = start.elapsed();
    println!("Done: {:?}", end);
}
