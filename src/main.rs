mod working_functions;
fn main() {
    println!(
        "result = {}, answer = {}",
        working_functions::check_exam(&["b", "c", "b", "a"], &["", "a", "a", "c"]),
        0
    );
}

