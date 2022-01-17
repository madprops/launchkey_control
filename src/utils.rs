pub fn get_percentage(data: &str) -> isize {
  ((data.parse::<f64>().unwrap() / 127.0) * 100.0) as isize
}