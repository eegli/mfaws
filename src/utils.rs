use std::time::{Duration, SystemTime};

pub fn get_remaining_time(time: SystemTime) -> Option<String> {
    time.duration_since(SystemTime::now())
        .ok()
        .map(format_duration)
}

fn format_duration(d: Duration) -> String {
    [
        (d.as_secs() / 3600),
        ((d.as_secs() % 3600) / 60),
        (d.as_secs() % 60),
    ]
    .iter()
    .zip(['h', 'm', 's'])
    .filter(|(n, _)| **n > 0)
    .fold(String::new(), |acc, (t, d)| acc + &format!("{}{} ", t, d))
    .trim_end()
    .to_string()
}

pub fn confirm_prompt(msg: &str) -> bool {
    println!("{} (y/yes/n/no)", msg);
    let mut input = String::new();
    if let Err(_) = std::io::stdin().read_line(&mut input) {
        return false;
    };
    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => true,
        _ => false,
    }
}

pub fn get_input(msg: &str) -> anyhow::Result<String> {
    let mut input = String::new();
    println!("{}", msg);
    std::io::stdin().read_line(&mut input)?;
    input = input.trim().to_owned();
    Ok(input)
}

#[cfg(test)]
mod test_utils {
    use super::*;

    #[test]
    fn test_format_duration_1() {
        let dur = Duration::from_secs(3600);
        assert_eq!(format_duration(dur), "1h".to_string());
    }
    #[test]
    fn test_format_duration_2() {
        let dur = Duration::from_secs(3599);
        assert_eq!(format_duration(dur), "59m 59s".to_string());
    }
    #[test]
    fn test_format_duration_3() {
        let dur = Duration::from_secs(59);
        assert_eq!(format_duration(dur), "59s".to_string());
    }
    #[test]
    fn test_format_duration_4() {
        let dur = Duration::from_secs(0);
        assert_eq!(format_duration(dur), "".to_string());
    }
    #[test]
    fn test_format_remaining() {
        let past_time = SystemTime::now() - Duration::from_secs(1);
        assert!(get_remaining_time(past_time).is_none());
    }
}
