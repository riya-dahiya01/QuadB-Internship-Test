/*
    Submission by Riya Dahiya
    email- dahiyariya2001@gmail.com
*/


// Function to check if a given string is a palindrome
fn is_palindrome(input: &str) -> bool {
    let input = input.to_lowercase();
    input.chars().eq(input.chars().rev())
}

// Function to find the index of the first occurrence of a given number in a sorted array
fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

// Function to return the shortest word in a string of words
fn shortest_word(words: &str) -> Option<&str> {
    words.split_whitespace().min_by_key(|word| word.len())
}

// Function to check if a given number is prime
fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

// Function to return the median of a sorted array of integers
fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        (arr[len / 2] + arr[len / 2 - 1]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

// Function to find the longest common prefix of a set of strings
fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let mut prefix = strings[0].clone();
    for string in &strings[1..] {
        prefix.truncate(prefix.chars().zip(string.chars()).take_while(|(a, b)| a == b).count());
    }
    prefix
}

// Function to return the kth smallest element in an array
fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let mut sorted_arr = arr.to_vec();
        sorted_arr.sort();
        Some(sorted_arr[k - 1])
    } else {
        None
    }
}

// Definition of a binary tree node
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

// Function to return the maximum depth of a binary tree
fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

// Reverse a string
fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

// Check if a number is prime
fn is_prime_rust(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

// Merge two sorted arrays
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }
    merged.extend_from_slice(&arr1[i..]);
    merged.extend_from_slice(&arr2[j..]);
    merged
}

// Find the maximum subarray sum
fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = 0;
    let mut current_sum = 0;
    for &num in arr {
        current_sum = current_sum.max(0) + num;
        max_sum = max_sum.max(current_sum);
    }
    max_sum
}

fn main() {
    // Test cases
    println!("Is 'radar' a palindrome? {}", is_palindrome("radar"));
    println!("Index of first occurrence of 5: {:?}", first_occurrence(&[1, 3, 5, 7, 9], 5));
    println!("Shortest word in 'apple orange banana': {:?}", shortest_word("apple orange banana"));
    println!("Is 17 prime? {}", is_prime(17));
    println!("Median of [1, 3, 5, 7, 9]: {}", median(&[1, 3, 5, 7, 9]));
    println!("Longest common prefix: {:?}", longest_common_prefix(&vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]));
    println!("3rd smallest element: {:?}", kth_smallest(&[1, 4, 2, 5, 3], 3));
    println!("Maximum depth of binary tree: {:?}", max_depth(Some(Box::new(TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            val: 20,
            left: Some(Box::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            })),
        })),
    }))));
    println!("Reverse of 'hello': {}", reverse_string("hello"));
    println!("Is 23 prime? {}", is_prime_rust(23));
    println!("Merged sorted arrays: {:?}", merge_sorted_arrays(&[1, 3, 5], &[2, 4, 6]));
    println!("Maximum subarray sum: {}", max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]));
}
