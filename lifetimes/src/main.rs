use std::fmt;
use std::string::ToString;

struct ArbitraryStruct {
    value: usize,
}

impl fmt::Display for ArbitraryStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} hehe", self.value)
    }
}

trait Appraisable {
    fn value(&self) -> usize;
}

impl<T> Appraisable for T
where
    T: ToString,
{
    fn value(&self) -> usize {
        self.to_string().len()
    }
}

fn costlier<'a, T: fmt::Display + Appraisable>(first: &'a T, second: &'a T) -> &'a T {
    println!(
        "'{}' valued at: {}, '{}' valued at: {}",
        first,
        first.value(),
        second,
        second.value()
    );
    if first.value() > second.value() {
        first
    } else {
        second
    }
}

fn appraise<'a, T: fmt::Display + Appraisable>(first: &'a T, second: &'a T) {
    println!("'{}' is more valuable.", costlier(&first, &second));
}

fn main() {
    let dynamic_string = "This is a dynamic string.".to_owned();
    let static_string = "This is a static string.";
    appraise(&"asd", &dynamic_string.as_str());
    appraise(
        &"This is a string wayyyy longer than the next one.",
        &static_string,
    );
    appraise(&1231, &2134968);
    appraise(
        &ArbitraryStruct { value: 10 },
        &ArbitraryStruct { value: 150 },
    );
}
