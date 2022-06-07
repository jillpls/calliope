extern crate crypto;
extern crate rand;

use rand::{rngs::{StdRng}, SeedableRng, RngCore};

pub struct PasswordHasher {
    salt_size: usize,
    cost: u32,
}

impl PasswordHasher {
    pub fn hash_password(&self, password: &[u8], output: &mut [u8]) {
        let mut salt = vec![0u8;self.salt_size];
        crypto::bcrypt::bcrypt(self.cost, &mut salt, password, output)
    }

    fn generate_salt(&self, salt : &mut [u8]) {
        let mut rng = StdRng::from_entropy();
        rng.fill_bytes(salt);
    }
}
