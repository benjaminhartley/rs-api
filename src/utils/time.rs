use std::time::{SystemTime, UNIX_EPOCH};

pub fn timestamp_ms() -> u128 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    now.as_millis()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_timestamp_ms() {
        use std::thread::sleep;
        use std::time::Duration;
        let now = super::timestamp_ms();

        let ten_millis = Duration::from_millis(10);
        sleep(ten_millis);

        let later = super::timestamp_ms();
        assert!(later > now);
    }
}
