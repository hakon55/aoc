use std::fs::File;
use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    //let path = "example.txt";
    let path = "input.txt";

    // Open the file
    let file = File::open(path).expect("Failed to open the file");

    let reader = io::BufReader::new(file);

    // Initialize two vectors to store the two columns of data
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    // Loop through each line, using expect to handle potential errors
    for line in reader.lines() {
        let line_content = line.expect("Failed to read a line");

        // Split the line by space and parse the two integers
        let parts: Vec<&str> = line_content.split_whitespace().collect();
        let num1: i32 = parts[0].parse().expect("Failed to parse first number");
        let num2: i32 = parts[1].parse().expect("Failed to parse second number");

        // Add the integers to their respective lists
        list1.push(num1);
        list2.push(num2);
    }
    // Sort both lists
    list1.sort();
    list2.sort();

    // Print the sorted lists
    println!("Sorted List 1: {:?}", list1);
    println!("Sorted List 2: {:?}", list2);

    // Part 1
    //let sum_of_differences: i32 = list1.iter().zip(list2.iter()).map(|(a, b)| (a - b).abs()).sum();

    //println!("Sum: {}", sum_of_differences);

    // Part 2
    let mut occurrences = HashMap::new();

    for &number in &list2 {
        *occurrences.entry(number).or_insert(0) += 1;
    }

    println!("{:#?}", occurrences);

        let sum: i32 = list1
        .iter()
        .map(|&num| num * occurrences.get(&num).unwrap_or(&0)) // Multiply by value in HashMap or 0
        .sum();

    println!("Summen: {}", sum);
}
