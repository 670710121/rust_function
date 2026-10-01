// exercise 1 Is even
fn is_even(number: i32){
    if number % 2 == 0 {
        println!("True");
    } else {
        println!("False");
    }
}

// exercise 2 Discount Calculator
fn grade_check(score: i32){
    if score >= 80 {
        println!("Exellent");
    }else if score >= 40 {
        println!("Pass");
    } else {
        println!("Fail");
    }
}

fn main() {
      // --- Exercise 1 ---
    println!("\n--- Exercise 1 ---");
    let num1 = 4;
    let num2 = 7;

    is_even(num1); // True
    is_even(num2); // False
    // --- Exercise 2 ---
    println!("--- Exercise 2 ---");
    grade_check(39); // Fail
    grade_check(40); // Pass
    grade_check(90); // Exellent
}