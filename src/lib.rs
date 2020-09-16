/// Generates a random password based on set length
/// 
/// # Examples
/// ```
/// let password = password::generate_password(30);
/// assert_eq!(password.len(), 30);
/// ```

use rand::Rng;

pub fn generate_password(length: usize) -> String {

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";

    let mut rng = rand::thread_rng();

    if length < 6 {
        panic!("need a length greater than 6");
    } else {
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_correct_password_length() {
        let password = generate_password(7);
        assert_eq!(password.len(), 7);
    }
    #[test]
    fn test_generate_unique_passwords() {
        let password1 = generate_password(15);
        let password2 = generate_password(15);
        assert_ne!(password1, password2);
    }

    #[test]
    #[should_panic(expected = "need a length greater than 6")]
    fn test_panic_length() {
        let password = generate_password(0);
        assert_eq!(password.len(), 0);
    }


}