use rand::Rng;

pub fn roll(n: u64) -> u64 {
    rand::thread_rng().gen_range(1..=n)
}
