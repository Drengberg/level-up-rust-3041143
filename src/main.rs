fn median(a: Vec<f32>) -> Option<f32> {
    if a.is_empty() {
        return None
    }
    let mut b = a.clone();
    if !b.is_sorted() {
        b.sort_by(f32::total_cmp);
    }
    let length = b.len();
    let mid = length/2;
    let odd = !length.is_multiple_of(2);
    let mut median =0.0f32;
    let lower = *b.get(mid-1)?;
    let upper = *b.get(mid)?;
    if odd {
        median = *b.get(mid)?;
        println!("odd {median}" );
    } else {

        median = (*b.get(mid-1)?+*b.get(mid)?)/2.0f32;
    }   println!("even {lower} + {upper} / 2 = {median}");
    Some(median)
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
