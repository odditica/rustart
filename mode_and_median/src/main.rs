use std::collections::HashMap;
use std::io;
use std::io::Write;

fn get_ints() -> Vec<i64> {
    return 'get_input: loop {
        print!("> ");
        io::stdout().flush().ok();

        let mut input: String = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            continue 'get_input;
        }
        let split_entries: Vec<&str> = input.split(",").collect();
        let list: Vec<i64> = split_entries
            .iter()
            .filter_map(|x| x.trim().parse::<i64>().ok())
            .collect();
        if split_entries.len() == list.len() {
            break list;
        }
        println!("Invalid list input.");
    };
}

fn get_mode(list: &[i64]) -> (i64, usize) {
    assert!(!list.is_empty());

    let mut occurences: HashMap<i64, usize> = HashMap::new();
    for value in list {
        let count = occurences.entry(*value).or_insert(0);
        *count += 1;
    }

    let mut mode = (0, 0);
    for entry in occurences {
        if entry.1 > mode.1 {
            mode = entry;
        }
    }

    return mode;
}

fn get_median(list: &[i64]) -> f64 {
    assert!(!list.is_empty());

    let count = list.len();
    // FIXME: test if sorted
    return if count % 2 == 0 {
        ((list[count / 2] + list[count / 2 - 1]) as f64) * 0.5
    } else {
        list[count / 2] as f64
    };
}

fn main() {
    loop {
        let mut integers = get_ints();
        integers.sort();

        let median = get_median(&integers);
        println!("Median is {}", median);

        let mode = get_mode(&integers);
        println!("Mode is {:?}.", mode);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_test() {
        use float_cmp::*;

        let list = vec![0, 2, 1, 3, 4];
        approx_eq!(f64, get_median(&list), 2.0, F64Margin::default());
        let list = vec![0, 1, 3, 2];
        approx_eq!(f64, get_median(&list), 1.5, F64Margin::default());
        let list = vec![0];
        approx_eq!(f64, get_median(&list), 0.0, F64Margin::default());
    }

    #[test]
    #[should_panic]
    fn median_test_empty() {
        let list: Vec<i64> = Vec::new();
        get_median(&list);
    }

    #[test]
    fn occurence_test() {
        let list = vec![0, 4, 2, 3, 1];
        assert_eq!(get_mode(&list).1, 1);
        let list = vec![0, 0, 1, 3, 2, 4];
        assert_eq!(get_mode(&list), (0, 2));
        let list = vec![0, 1, 2, 3, 3, 3, 3, 4];
        assert_eq!(get_mode(&list), (3, 4));
        let list = vec![0];
        assert_eq!(get_mode(&list), (0, 1));
    }

    #[test]
    #[should_panic]
    fn occurence_test_empty() {
        let list: Vec<i64> = Vec::new();
        get_mode(&list);
    }
}
