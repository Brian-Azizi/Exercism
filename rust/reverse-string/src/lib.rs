pub fn reverse(input: &str) -> String {
    return input.split_inclusive("").rev().collect();
}
