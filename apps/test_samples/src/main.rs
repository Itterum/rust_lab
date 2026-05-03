use regex::Regex;

fn main() {
    let reg_text = |text: &str| {
        let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();

        if let Some(caps) = re.captures(text) {
            println!("Year: {}", &caps[1]);
            println!("Month: {}", &caps[2]);
            println!("Day: {}", &caps[3]);
        }
    };

    let date = "2024-05-03";

    reg_text(date);
}
