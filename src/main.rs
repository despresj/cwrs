#[allow(unused_variables)]
mod working_functions;

fn main() {
    println!(
        "result = {}, answer = {}",
        working_functions::dna_strand("AAAA"),
        "TTTT"
    );
}
