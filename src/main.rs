use chrono::{DateTime, Local};
use chrono::{Duration, TimeZone};

struct ImportantEvent {
    what: String,
    when: DateTime<Local>,
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        let passed = *&self.when < Local::now();
        println!("Value: {}", passed);
        passed
    }
}

fn main() {
    let missed_christmas = ImportantEvent {
        what: String::from("Christmas"),
        when: chrono::Local.with_ymd_and_hms(2026, 12, 24, 0, 0, 0).unwrap()
    };

    if missed_christmas.is_passed() {
        println!("oh well, maybe next year");
    } else {
        println!("☃︎");
    }
}

#[test]
fn in_past() {
    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: chrono::Local::now() - Duration::hours(25),
    };

    assert!(event.is_passed())
}

#[test]
fn in_future() {
    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: chrono::Local::now() + Duration::hours(25),
    };

    assert!(!event.is_passed())
}
