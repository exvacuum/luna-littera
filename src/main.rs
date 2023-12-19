use chrono::prelude::*;

const FIXED_LUNAR_CYCLE: i64 = 2551443;

const CYCLE_CHARS: [char; 28] = [
    '','','','','','','',
    '','','','','','','',
    '','','','','','','',
    '','','','','','',''
];

fn main() {
    let start = Utc.ymd(2022, 7, 28).and_hms(17, 54, 00);
    let now = Utc::now();

    let duration = now - start;
    let duration = duration.num_seconds() % FIXED_LUNAR_CYCLE;
    let cycle_factor = duration as f64/FIXED_LUNAR_CYCLE as f64;
    let cycle_char = (cycle_factor * CYCLE_CHARS.len() as f64) as usize;
    println!("{}", CYCLE_CHARS[cycle_char]);
}
