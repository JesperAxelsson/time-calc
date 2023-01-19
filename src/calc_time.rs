use regex::Regex;

pub fn calc_time(text: &str) -> String {
    let lines = text.lines();

    let mut buf = String::new();

    for line in lines {
        let times: Vec<u64> = line.split_whitespace().filter_map(get_time).collect();

        if times.len() < 2 {
            buf.push_str(&format!("At least two times needed! {:?}:", times.len()));
            buf.push_str(line);
            buf.push_str("\n");
            continue;
        }

        if times.len() % 2 == 1 {
            buf.push_str(&format!(
                "Odd number of times! {:?} {:?}:",
                times.len() % 2,
                times.len()
            ));
            buf.push_str(line);
            buf.push_str("\n");
            continue;
        }

        if times.as_slice().windows(2).any(|arr| arr[0] >= arr[1]) {
            buf.push_str(&format!("Preceeding number be higher then last! ",));
            buf.push_str(line);
            buf.push_str("\n");
            continue;
        }

        let mut time_sum = 0;
        for time in times.as_slice().chunks(2) {
            time_sum += time[1] - time[0];
        }

        buf.push_str(&format!("{:.2}", time_sum as f64 / 60.).replace(".", ","));
        buf.push_str("\n");
    }

    return buf;
}

/// Returns time in minutes
fn get_time(time: &str) -> Option<u64> {
    let time_reg = Regex::new(r"^(\d{1,2}):(\d{2})$").unwrap();
    if let Some(caps) = time_reg.captures(time) {
        if caps.len() == 3 {
            let hour = caps.get(1)?.as_str().parse::<u64>().ok()?;
            let minutes = caps.get(2)?.as_str().parse::<u64>().ok()?;

            if hour > 24 {
                return None;
            }
            if minutes > 59 {
                return None;
            }

            return Some(hour * 60 + minutes);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use crate::calc_time::*;

    #[test]
    fn test_normal_time() {
        let result = "08:30";
        assert_eq!(get_time(result), Some(510));
    }

    #[test]
    fn test_no_zero_time() {
        let result = "8:30";
        assert_eq!(get_time(result), Some(510));
    }

    #[test]
    fn test_edge_high_time() {
        let result = "24:59";
        assert_eq!(get_time(result), Some(1499));
    }

    #[test]
    fn test_edge_low_time() {
        let result = "00:00";
        assert_eq!(get_time(result), Some(0));
    }

    #[test]
    fn test_bad_hour_time() {
        let result = "25:30";
        assert_eq!(get_time(result), None);
    }

    #[test]
    fn test_bad_minute_time() {
        let result = "08:61";
        assert_eq!(get_time(result), None);
    }
}
