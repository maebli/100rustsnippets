fn dna_strand(dna: &str) -> String {
    dna.chars().fold(String::new(), |mut acc, inst| {
        match inst {
            'T' => acc.push('A'),
            'A' => acc.push('T'),
            'G' => acc.push('C'),
            'C' => acc.push('G'),
            _x => panic!("wrongly formatted dna sequence"),
        }
        acc
    })
}

fn main() {}

#[test]
fn returns_expected() {
    assert_eq!(dna_strand("AAAA"), "TTTT");
    assert_eq!(dna_strand("ATTGC"), "TAACG");
    assert_eq!(dna_strand("GTAT"), "CATA");
}
