fn unique(a: Vec<i32>) -> Vec<i32> {
    if a.is_empty() {
        return a;
    }

    let mut found = false;
    let mut b: Vec<i32> = vec![a[0]];
    for a_nbr in &a {
        for b_nbr in &b {
            if a_nbr == b_nbr {
                found = true;
            }
        }
        if !found {
            b.push(*a_nbr);
        }
        found = false;
    }
    return b;
}

// advanced 1: use generic types
fn unique1<T: std::cmp::PartialEq + Copy>(a: Vec<T>) -> Vec<T> {
    if a.is_empty() {
        return a;
    }

    let mut found = false;
    let mut b: Vec<T> = vec![a[0]];
    for a_nbr in &a {
        for b_nbr in &b {
            if a_nbr == b_nbr {
                found = true;
            }
        }
        if !found {
            b.push(*a_nbr);
        }
        found = false;
    }
    return b;
}

// advanced 2: keep items in order
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }

// advanced 3: use iterators
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }

fn main() {
    let input = vec![2, 1, 1];
    let answer = unique1(input);
    println!("unique items -> {:?}", answer)
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = vec![];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let mut expected_output = vec![1, 4, 5];
    let mut actual_output = unique1(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique1(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique1(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique1(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}
