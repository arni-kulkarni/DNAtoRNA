fn transcribe_dna_to_rna(dna: &str) -> String {
    let mut rna = String::new();
    for c in dna.chars() {
        match c {
            'T' => rna.push('U'),
            _ => rna.push(c),
        }
    }
    rna
}

fn main() {
    println!("Enter a DNA sequence:");
    let mut dna = String::new();
    std::io::stdin().read_line(&mut dna).unwrap();
    let dna = dna.trim(); 

    let rna = transcribe_dna_to_rna(dna);
    println!("RNA: {}", rna);
}
#[test]
fn test_transcribe() {
    assert_eq!(transcribe_dna_to_rna("ATGC"), "AUGC");
}
