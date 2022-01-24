use chrono::{DateTime, TimeZone, Utc};

pub fn vec_to_null_terminated_str(buf: Vec<u8>) -> String {
    String::from_utf8_lossy(&buf)
        .to_string()
        .split('\0')
        .next()
        .unwrap()
        .into()
}

pub fn string_to_vec(str: &str, length: usize) -> Vec<u8> {
    let mut str_chars = str.as_bytes().iter();
    (0..length)
        .map(|_| *str_chars.next().unwrap_or(&0u8))
        .collect()
}

pub fn epoch_to_chrono(epoch: u32) -> DateTime<Utc> {
    Utc.timestamp(epoch as i64, 0)
}

pub fn chrono_to_epoch(datetime: &DateTime<Utc>) -> u32 {
    datetime.timestamp() as u32
}
