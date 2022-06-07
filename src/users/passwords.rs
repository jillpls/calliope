extern crate argon2;
extern crate rand;

use argon2 as a2;
use rand::{rngs::StdRng, RngCore, SeedableRng};

pub fn hash_password(
    password: &[u8],
    salt: Option<&[u8]>,
    salt_length: Option<usize>,
) -> Result<String, a2::Error> {
    let salt = &generate_salt(salt, salt_length);
    let config = a2::Config::default();
    a2::hash_encoded(password, salt, &config)
}

fn generate_salt(salt: Option<&[u8]>, salt_length: Option<usize>) -> Vec<u8> {
    match salt {
        Some(s) => {
            let mut padded = s.to_vec();
            if s.len() < 8 {
                padded.extend(std::iter::repeat(0).take(8 - s.len()));
            }
            padded
        }
        None => {
            let mut salt_length = salt_length.unwrap_or(16);
            if salt_length < 8 {
                salt_length = 8;
            }
            let mut salt = vec![0u8; salt_length];
            let mut rng = StdRng::from_entropy();
            rng.fill_bytes(&mut salt);
            salt
        }
    }
}

#[cfg(test)]
mod tests {
    use super::hash_password;

    #[test]
    fn hash() {
        let hashed_password = hash_password(b"password", Some(b"saltx"), None).unwrap();
        let hashed_password2 = hash_password(b"aaapassword", Some(b"saltx"), None).unwrap();
        assert_eq!(hashed_password, hashed_password2);
    }
}
