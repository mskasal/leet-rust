use std::{cmp, collections::HashMap};

fn main() {
    let _test_this = test_this("this is a str");
    let _remove_duplicates = remove_duplicates(&mut vec![1, 1, 2]);
    let _remove_duplicates_ii = remove_duplicates_ii(&mut vec![1, 1, 2, 2, 2, 3]);
    let _max_profit = max_profit(vec![7, 1, 5, 3, 6, 4]);
    let _valid_palindrome = valid_palindrome(String::from("this is a str"));
    let _rotate_array = rotate_array(&mut vec![1, 2, 3, 4, 5, 6, 7], 3);
    let _roman_numbers = roman_numbers("MCMXCIV".to_string());
    let _length_of_last_word = length_of_last_word("    day".to_string());
    let _is_subsequence = is_subsequence("bb".to_string(), "ahbgdc".to_string());
}

fn test_this(str: &str) -> String {
    str.to_string()
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut left = 0;
    for right in 1..nums.len() {
        if nums[left] != nums[right] {
            nums[left + 1] = nums[right];
            left += 1;
        }
    }
    left as i32 + 1
}

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut major = 0;
    let mut count = 0;

    for num in nums.iter() {
        if count == 0 {
            major = *num;
            count += 1;
        } else if major == *num {
            count += 1;
        } else {
            count -= 1;
        }
    }

    major
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut sell = 1;
    let mut buy = 0;
    let mut max_profit = 0;

    while sell < prices.len() {
        if prices[buy] < prices[sell] {
            max_profit = cmp::max(prices[sell] - prices[buy], max_profit);
        } else {
            buy = sell;
        }

        sell += 1;
    }
    max_profit
}

fn remove_duplicates_ii(nums: &mut Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums.len() as i32;
    }

    let mut slow = 2;

    for fast in 2..nums.len() {
        if nums[slow - 2] != nums[fast] {
            nums[slow] = nums[fast];
            slow += 1
        }
    }

    slow as i32
}

fn valid_palindrome(s: String) -> bool {
    let s = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<Vec<_>>();

    if s.is_empty() {
        return true;
    }

    let mut left = 0;
    let mut right = s.len() - 1;

    while left < right {
        if s[left].to_ascii_lowercase() != s[right].to_ascii_lowercase() {
            return false;
        }

        left += 1;
        right -= 1;
    }

    true
}

fn rotate_array(nums: &mut Vec<i32>, k: i32) -> i32 {
    let mut i = 0;

    while i < k {
        if let Some(popped) = nums.pop() {
            nums.insert(0, popped);
            i += 1;
        }
    }
    k
}

fn roman_numbers(s: String) -> i32 {
    let mut pairs: HashMap<char, i32> = HashMap::new();
    let mut result = 0;
    let mut iter = s.chars().peekable();

    pairs.insert('I', 1);
    pairs.insert('V', 5);
    pairs.insert('X', 10);
    pairs.insert('L', 50);
    pairs.insert('C', 100);
    pairs.insert('D', 500);
    pairs.insert('M', 1000);

    while let Some(current) = iter.next() {
        match iter.peek() {
            Some(&next) => {
                if pairs[&next] > pairs[&current] {
                    result -= pairs[&current];
                } else {
                    result += pairs[&current];
                }
            }
            _ => {
                result += pairs[&current];
            }
        }
    }

    result
}

fn length_of_last_word(s: String) -> i32 {
    let trimmed = s.trim();
    if trimmed.len() == 1 {
        return 1;
    }

    let mut j = 0;
    for i in 0..trimmed.len() {
        if let Some(char) = trimmed.chars().nth(trimmed.len() - 1 - i) {
            if char == ' ' {
                break;
            }
            j += 1;
        }
    }

    j
}

fn is_subsequence(s: String, t: String) -> bool {
    let mut index = 0;

    for char_t in t.chars() {
        if let Some(char_s) = s.chars().nth(index) {
            if index < s.len() && char_s == char_t {
                index += 1
            }
        }
    }
    index >= s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_this() {
        let str = "test";
        let result = test_this(str);

        assert_eq!(result, str.to_string());
    }

    #[test]
    fn test_remove_duplicates() {
        let mut numbers: Vec<i32> = Vec::from([1, 1, 2, 2, 3]);
        let expected_nums = Vec::from([1, 2, 3]);
        let result = remove_duplicates(&mut numbers);
        assert_eq!(result, 3);
        assert_eq!(numbers[0..3], expected_nums);
    }

    #[test]
    fn test_remove_duplicates_ii() {
        let mut numbers: Vec<i32> = Vec::from([1, 1, 2, 2, 2, 3]);
        let expected_nums = Vec::from([1, 1, 2, 2, 3]);
        let result = remove_duplicates_ii(&mut numbers);
        assert_eq!(result, 5);
        assert_eq!(numbers[0..5], expected_nums);
    }

    #[test]
    fn test_majority_element() {
        let nums = Vec::from([3, 2, 3]);
        let result = majority_element(nums);

        assert_eq!(result, 3);
    }

    #[test]
    fn test_max_profit() {
        let prices = Vec::from([7, 3, 5, 1, 6, 4]);

        let result = max_profit(prices);

        assert_eq!(result, 5);
    }

    #[test]
    fn test_is_subsequence() {
        let string1 = String::from("abc");
        let string2 = String::from("abhgdc");
        let string3 = String::from("axc");
        let string4 = String::from("abhgdc");
        let string5 = String::from("bb");
        let string6 = String::from("abhgdc");

        let result = is_subsequence(string1, string2);
        let result2 = is_subsequence(string3, string4);
        let result3 = is_subsequence(string5, string6);

        assert!(result);
        assert!(!result2);
        assert!(!result3);
    }

    #[test]
    fn test_valid_palindrome() {
        let string = String::from("A man, a plan, a canal: Panama");
        let string_two = String::from("asd");
        let string_three = String::from("race a car");
        let string_four = String::from("a.");

        let result = valid_palindrome(string);
        let result_two = valid_palindrome(string_two);
        let result_three = valid_palindrome(string_three);
        let result_four = valid_palindrome(string_four);

        assert!(result);
        assert!(!result_two);
        assert!(!result_three);
        assert!(result_four);
    }

    #[test]
    fn test_rotate_array() {
        let mut nums = Vec::from([1, 2, 3, 4, 5, 6, 7]);

        let _result = rotate_array(&mut nums, 3);
        let expected_nums = Vec::from([5, 6, 7, 1, 2, 3, 4]);

        assert_eq!(nums, expected_nums);
    }

    #[test]
    fn test_roman_numbers() {
        let roman_numer_string_one = String::from("III");
        let roman_numer_string_two = String::from("LVIII");
        let roman_numer_string_three = String::from("MCMXCIV");
        let result = roman_numbers(roman_numer_string_one);
        let result_two = roman_numbers(roman_numer_string_two);
        let result_three = roman_numbers(roman_numer_string_three);

        assert_eq!(result, 3);
        assert_eq!(result_two, 58);
        assert_eq!(result_three, 1994);
    }
    #[test]
    fn test_length_of_last_word() {
        let sentence = "Hellow world".to_string();
        let sentence_two = "   fly me   to   the moon  ".to_string();
        let sentence_three = "    day".to_string();

        let result = length_of_last_word(sentence);
        let result_two = length_of_last_word(sentence_two);
        let result_three = length_of_last_word(sentence_three);

        assert_eq!(result, 5);
        assert_eq!(result_two, 4);
        assert_eq!(result_three, 3);
    }
}
