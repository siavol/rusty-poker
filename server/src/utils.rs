use rand::{distributions::Alphanumeric, Rng};

pub fn generate_uid() -> String {
    const UID_LEN: usize = 7;

    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(UID_LEN)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::generate_uid;

    #[test]
    fn generate_uid_returns_string() {
        let uid = generate_uid();
        assert!(uid.len() > 0);
    }
}