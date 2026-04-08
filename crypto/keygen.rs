use rand::Rng;

pub fn generate_key() -> String {
    let mut rng = rand::thread_rng();
    (0..64).map(|_| format!("{:x}", rng.gen::<u8>())).collect()
}
