

fn main_calls() {
    println!("Main calls me");
}

fn test_1_calls() {
    println!("First test calls me");
}

fn test_2_calls() {
    println!("Second test calls me");
}

fn main() {
    main_calls();
}

#[test]
fn test_1() {
    test_1_calls();
}

#[test]
fn test_2() {
    test_2_calls();
}
