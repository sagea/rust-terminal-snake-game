use rand::Rng;

pub fn char_len(str: &String) -> i32 {
  str.chars().count() as i32
}

pub fn random_between(a: i32, b: i32) -> i32 {
  let mut rng = rand::thread_rng();
  rng.gen_range(a..b)
}
