use std::{cmp, collections::HashMap, usize};

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
    let _tow_sum = two_sum(vec![2, 7, 11, 15], 9);
    let _max_area = max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
    let _three_sum = three_sum(vec![-1, 0, 1, 2, -1, -4]);
    let _can_construct = can_construct("aa".to_string(), "aab".to_string());
    let _is_isomorphic = is_isomorphic("egg".to_string(), "add".to_string());
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

fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left: usize = 0;
    let mut right: usize = numbers.len() - 1;
    let mut sum: i32;

    loop {
        sum = numbers[left] + numbers[right];

        if sum == target {
            break;
        }

        if sum < target {
            left += 1;
        }

        if sum > target {
            right -= 1;
        }
    }

    vec![left as i32 + 1, right as i32 + 1]
}

fn max_area(height: Vec<i32>) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = height.len() - 1;
    let mut max_area: i32 = 0;

    loop {
        let area = (right - left) as i32 * height[left].min(height[right]);

        if max_area < area {
            max_area = area;
        }

        if height[left] >= height[right] {
            right -= 1;
        } else {
            left += 1;
        }
        if left >= right {
            break;
        }
    }

    max_area
}

fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
        return vec![vec![]];
    }

    if nums.len() == 3 {
        let sum: i32 = nums.iter().sum();
        if sum == 0 {
            return vec![nums];
        } else {
            return vec![vec![]];
        }
    }

    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut nums_sorted = nums.clone();
    nums_sorted.sort();

    let mut left: usize;
    let mut right: usize;

    for i in 0..nums_sorted.len() - 2 {
        left = i + 1;
        right = nums_sorted.len() - 1;

        let target = -nums_sorted[i];

        loop {
            if left >= right {
                break;
            }
            if nums_sorted[right] == nums[right - 1] {
                right -= 1;
                continue;
            }
            if nums_sorted[left] == nums[left + 1] {
                left += 1;
                continue;
            }
            let sum = nums_sorted[left] + nums_sorted[right];

            if sum == target {
                if !result.contains(&vec![nums_sorted[i], nums_sorted[left], nums_sorted[right]]) {
                    result.push(vec![nums_sorted[i], nums_sorted[left], nums_sorted[right]]);
                }
                right -= 1;
            }

            if sum < target {
                left += 1;
            }

            if sum > target {
                right -= 1;
            }
        }
    }

    result
}

fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut mag_char_map: HashMap<char, usize> = HashMap::new();
    let mut result: bool = false;

    if ransom_note.len() > magazine.len() {
        return result;
    }

    for char_m in magazine.chars() {
        *mag_char_map.entry(char_m).or_insert(0) += 1;
    }

    for char_r in ransom_note.chars() {
        if let Some(ent) = mag_char_map.get_mut(&char_r) {
            if *ent > 0 {
                *ent -= 1;
                result = true;
            } else {
                result = false;
                break;
            }
        } else {
            result = false;
            break;
        }
    }

    result
}

fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut result = true;
    let mut s_map: HashMap<char, char> = HashMap::new();

    for i in 0..s.len() {
        if let Some(s_char) = s.chars().nth(i) {
            if let Some(t_char) = t.chars().nth(i) {
                if s_map.contains_key(&s_char) {
                    if s_map[&s_char] != t_char {
                        result = false;
                        break;
                    }
                } else {
                    if s_map.values().any(|&v| v == t_char) {
                        result = false;
                        break;
                    }
                    s_map.insert(s_char, t_char);
                }
            }
        }
    }

    result
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

    #[test]
    fn test_two_sum() {
        let numbers = Vec::from([2, 7, 11, 15]);
        let target = 9;
        let result = two_sum(numbers, target);

        let numbers_2 = Vec::from([2, 3, 4]);
        let target_2 = 6;
        let result_2 = two_sum(numbers_2, target_2);

        let numbers_3 = Vec::from([-1, 0]);
        let target_3 = -1;
        let result_3 = two_sum(numbers_3, target_3);

        assert_eq!(result, vec![1, 2]);
        assert_eq!(result_2, vec![1, 3]);
        assert_eq!(result_3, vec![1, 2]);
    }

    #[test]
    fn test_max_area() {
        let height = Vec::from([1, 8, 6, 2, 5, 4, 8, 3, 7]);
        let result = max_area(height);

        let height_1 = Vec::from([1, 1]);
        let result_1 = max_area(height_1);

        assert_eq!(result, 49);
        assert_eq!(result_1, 1);
    }

    #[test]
    fn test_three_sum() {
        let nums = Vec::from([-1, 0, 1, 2, -1, -4]);
        let nums_2 = Vec::from([0, 1, 1]);
        let nums_3 = Vec::from([0, 0, 0]);

        let result = three_sum(nums);
        let result_2 = three_sum(nums_2);
        let result_3 = three_sum(nums_3);

        assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
        assert_eq!(result_2, vec![[]]);
        assert_eq!(result_3, vec![[0, 0, 0]]);
    }

    #[test]
    fn test_can_construct() {
        let ransom_note = String::from("a");
        let magazine = String::from("b");

        let ransom_note_two = String::from("aa");
        let magazine_two = String::from("ab");

        let ransom_note_three = String::from("aa");
        let magazine_three = String::from("aab");

        let result = can_construct(ransom_note, magazine);
        let result_two = can_construct(ransom_note_two, magazine_two);
        let result_three = can_construct(ransom_note_three, magazine_three);

        assert!(!result);
        assert!(!result_two);
        assert!(result_three);
    }

    #[test]
    fn test_is_isomorphic() {
        let s = String::from("egg");
        let t = String::from("add");

        let s_two = String::from("foo");
        let t_two = String::from("bar");

        let s_three = String::from("paper");
        let t_three = String::from("title");

        let s_fourth = String::from("bbbaaaba");
        let t_fourth = String::from("aaabbbba");

        let s_fifth = String::from("badc");
        let t_fifth = String::from("baba");

        let result = is_isomorphic(s, t);
        let result_two = is_isomorphic(s_two, t_two);
        let result_three = is_isomorphic(s_three, t_three);
        let result_fourth = is_isomorphic(s_fourth, t_fourth);
        let result_fifth = is_isomorphic(s_fifth, t_fifth);

        assert!(result);
        assert!(!result_two);
        assert!(result_three);
        assert!(!result_fourth);
        assert!(!result_fifth);
    }
}
