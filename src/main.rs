fn main_calls() {
    println!("Main calls me");
    test_1_calls();
    test_2_calls();
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
    for _ in 0..1024 {
        test_1_calls();
    }
}

#[test]
fn test_2() {
    for i in 0..2048 {
        if i % 2 == 0 {
            test_2_calls();
        } else if false {
            test_1_calls();
        }
    }
}
