use std::io;

fn counting(vec: &Vec<char>, mut left: isize, mut right: isize) -> (isize, isize) {
    while left >= 0 && right < vec.len() as isize && vec[left as usize] == vec[right as usize] {
        left -= 1;
        right += 1;
    }
    (left + 1, right)
}

fn longest_polindrom(input_str: &str) -> &str {
    let vec: Vec<_> = input_str.chars().collect();
    let mut max_size_of_palindrome: (isize, isize) = (0, 0);

    for i in 0..vec.len() - 1 {
        let size_palindrome_odd = counting(&vec, i as isize, i as isize + 1);
        let size_palindrome_even = counting(&vec, i as isize, i as isize);
        let max_of_even_or_odd: (isize, isize);
        if size_palindrome_even.1 - size_palindrome_even.0
            > size_palindrome_odd.1 - size_palindrome_odd.0
        {
            max_of_even_or_odd = size_palindrome_even;
        } else {
            max_of_even_or_odd = size_palindrome_odd
        }

        if max_of_even_or_odd.1 - max_of_even_or_odd.0
            > max_size_of_palindrome.1 - max_size_of_palindrome.0
        {
            max_size_of_palindrome = max_of_even_or_odd;
        }
    }

    &input_str[(max_size_of_palindrome.0 as usize)..(max_size_of_palindrome.1 as usize)]
}

fn main() {
    let mut input_str = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .expect("Error in input!");

    println!("{}", longest_polindrom(&mut input_str));
}
