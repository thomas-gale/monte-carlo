use rand::prelude::*;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

///
/// Random float between 0 and 1
///
pub fn random_float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

///
/// Random int in range
///
pub fn random_int(min: u32, max: u32) -> u32 {
    (random_float() * ((max - min + 1) as f32)) as u32 + min
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_random_int() {
        for _ in 0..10 {
            let r_int = random_int(0, 2);
            assert!(r_int <= 2);
            // println!("Test Random Int {:?}", r_int)
        }
    }
}
