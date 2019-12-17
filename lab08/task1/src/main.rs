fn main() {
  println!("Hello, world!");
}

fn dna_strand(dna: &str) -> String {
  dna.chars().map(|x| match x {
    'A' => 'T',
    'T' => 'A',
    'G' => 'C',
    'C' => 'G',
    _ => x,
  }).collect()
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[cfg(test)]
mod tests {
  use super::dna_strand;

  #[test]
  fn returns_expected1() {
    assert_eq!(dna_strand("AAAA"), "TTTT");
  }

  #[test]
  fn returns_expected2() {
    assert_eq!(dna_strand("ATTGC"), "TAACG");
  }

  #[test]
  fn returns_expected3() {
    assert_eq!(dna_strand("GTAT"), "CATA");
  }

  #[test]
  fn returns_expected4() {
    assert_eq!(dna_strand("A"), "T");
  }

  #[test]
  fn returns_expected5() {
    assert_eq!(dna_strand("T"), "A");
  }

  #[test]
  fn returns_expected6() {
    assert_eq!(dna_strand("G"), "C");
  }


  #[test]
  fn returns_expected7() {
    assert_eq!(dna_strand("C"), "G");
  }


  #[test]
  fn returns_expected8() {
    assert_eq!(dna_strand(""), "");
  }


  #[test]
  fn returns_expected9() {
    assert_eq!(dna_strand("W"), "W");
  }


  #[test]
  fn returns_expected10() {
    assert_eq!(dna_strand("GTCA"), "CAGT");
  }
}
