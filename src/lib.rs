extern crate regex;

use regex::Regex;

/// Get the year from a YYYY-MM-DD Formatted Data
///
/// ```rust
/// # extern crate get_year;
/// # use get_year::get_year;
/// assert_eq!("2010".to_string(), get_year("2010-03-14".to_string()))
/// ```
pub fn get_year(date: String) -> String {
    let re = Regex::new(r"(?x)
(?P<year>\d{4})  # the year
-
(?P<month>\d{2}) # the month
-
(?P<day>\d{2})   # the day
").unwrap();
    let caps = re.captures(date.as_str()).unwrap();

    String::from(&caps["year"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_2010() {
        assert_eq!("2010".to_string(), get_year("2010-03-14".to_string()))
    }
    #[test]
    fn gets_1981() {
        assert_eq!("1981".to_string(), get_year("1981-13-12".to_string()))
    }
}