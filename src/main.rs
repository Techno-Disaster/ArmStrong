use std::io;
fn main() {
    println!("Enter a number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let input_num: i32 = input.trim().parse().expect("Not a valid number");
    
    println!("{}", checkforarmstrong(input_num));
//     println!("{}", checkforarmstrong(9));
//     println!("{}", checkforarmstrong(10));
//     println!("{}", checkforarmstrong(153));
//     println!("{}", checkforarmstrong(154));
} //numbers from gci task.

fn checkforarmstrong(number: i32) -> bool {
    let splitted_numbers: Vec<char> = number.to_string().chars().collect();
    let numbers = makeinteger(splitted_numbers);
    return addafterconverting(numbers) == number;
}

fn addafterconverting(digits: Vec<i32>) -> i32 {
    let mut sum = 0;
    let power = digits.len() as u32; //unsigned integer
    for num in digits {
        sum = sum + num.pow(power);
    }
    return sum;
}

fn makeinteger(chars: Vec<char>) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    for raw_number in chars {
        numbers.push(raw_number.to_digit(10).unwrap() as i32);
    }
    return numbers;
}
