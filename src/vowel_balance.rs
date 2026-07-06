///Given a string, determine whether the number of vowels in the first half of the string is equal to the number of vowels in the second half.

///The string can contain any characters.
///The letters a, e, i, o, and u, in either uppercase or lowercase, are considered vowels.
///If there's an odd number of characters in the string, ignore the center character.
///Tests:
///1. is_balanced("racecar") should return True.
///2. is_balanced("Lorem Ipsum") should return True.
///3. is_balanced("Kitty Ipsum") should return False.
///4. is_balanced("string") should return False.
///5. is_balanced(" ") should return True.
///6. is_balanced("abcdefghijklmnopqrstuvwxyz") should return False.
///7. is_balanced("123A#b!E&*456-o.U") should return True.

fn is_balanced(s: &str) -> bool {
    let vowels = "aeiouAEIOU";
    let len = s.len();
    let mid = len / 2;

    let first_half = &s[..mid];
    let second_half = if len % 2 == 0 {
        &s[mid..]
    } else {
        &s[mid + 1..]
    };

    let count_vowels = |s: &str| s.chars().filter(|c| vowels.contains(*c)).count();

    count_vowels(first_half) == count_vowels(second_half)
}

fn main() {
    let test_cases = [
        ("racecar", true),
        ("Lorem Ipsum", true),
        ("Kitty Ipsum", false),
        ("string", false),
        (" ", true),
        ("abcdefghijklmnopqrstuvwxyz", false),
        ("123A#b!E&*456-o.U", true),
    ];

    for (input, expected) in test_cases.iter() {
        let result = is_balanced(input);
        println!("is_balanced(\"{}\") = {} (expected: {})", input, result, expected);
    }
}