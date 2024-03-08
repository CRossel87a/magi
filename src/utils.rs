use chrono::Utc;


pub fn printf(s: &str) {
    let now = &Utc::now().format("%Y-%m-%d %H:%M:%S,%f").to_string()[0..24];
    let output = format!("{0} {1}", now, s);
    println!("{}",output);
}