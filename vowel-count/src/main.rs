fn get_count(string: &str) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    string.chars().filter(|c| vowels.contains(&c)).count()
}

fn main() {
    assert_eq!(get_count("abracadabra"), 5);
}
