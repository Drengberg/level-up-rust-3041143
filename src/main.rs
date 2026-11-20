fn sort_usernames<T: AsRef<str> + PartialOrd>(usernames: &mut Vec<T>) {
    //let length = usernames.len();
    //for _i in 1..=length {
    usernames.sort_by(|a, b| a.partial_cmp(b).unwrap());
    //}
}

fn main() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];

    println!("unsorted: {:?}", &users);
    sort_usernames(&mut users);
    println!("sorted:   {:?}", &users);
}

#[test]
fn five_users() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    let sorted = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
    sort_usernames(&mut users);

    assert_eq!(users, sorted);
}
