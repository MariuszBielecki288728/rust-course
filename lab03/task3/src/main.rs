use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Cipher {
  cipher_encode_map: HashMap<char, char>,
  cipher_decode_map: HashMap<char, char>,
}

impl Cipher {
  fn new(map1: &str, map2: &str) -> Cipher {
    let m: HashMap<char, char> = map1.chars().zip(map2.chars()).collect();
    let n: HashMap<char, char> = map2.chars().zip(map1.chars()).collect();
    Cipher { cipher_encode_map: m, cipher_decode_map: n }
  }
  
  fn encode(&self, string: &str) -> String {
    string.chars().map(
        |ch| self.cipher_encode_map.get(&ch).unwrap_or(&ch).to_owned()).collect()
  }
  
  fn decode(&self, string: &str) -> String {
    string.chars().map(
        |ch| self.cipher_decode_map.get(&ch).unwrap_or(&ch).to_owned()).collect()
  }
}


#[test]
fn example1() {
  let cipher = Cipher::new(
  "abcdefghijklmnopqrstuvwxyz", "etaoinshrdlucmfwypvbgkjqxz");
  assert_eq!(cipher.encode("abc"), "eta");
}

#[test]
fn example2() {
  let cipher = Cipher::new(
  "abcdefghijklmnopqrstuvwxyz", "etaoinshrdlucmfwypvbgkjqxz");
  assert_eq!(cipher.encode("xyz"), "qxz");
}

#[test]
fn example3() {
  let cipher = Cipher::new(
  "abcdefghijklmnopqrstuvwxyz", "etaoinshrdlucmfwypvbgkjqxz");
  assert_eq!(cipher.decode("eirfg"), "aeiou");
}

#[test]
fn example4() {
  let cipher = Cipher::new(
  "abcdefghijklmnopqrstuvwxyz", "etaoinshrdlucmfwypvbgkjqxz");
  assert_eq!(cipher.decode("erlang"), "aikcfu");
}

#[test]
fn example5() {
  let cipher = Cipher::new(
  "abc", "bcd");
  assert_eq!(cipher.encode("abc"), "bcd");
}

#[test]
fn example6() {
  let cipher = Cipher::new(
  ":", "-");
  assert_eq!(cipher.encode(":"), "-");
}

#[test]
fn example7() {
  let cipher = Cipher::new(
  "1", "2");
  assert_eq!(cipher.decode("2"), "1");
}

#[test]
fn example8() {
  let cipher = Cipher::new(
  "ą", "a");
  assert_eq!(cipher.encode("ą"), "a");
}

#[test]
fn example9() {
  let cipher = Cipher::new(
  "a", "z");
  assert_eq!(cipher.encode("a"), "z");
}

#[test]
fn example10() {
  let cipher = Cipher::new(
  "123", "123");
  assert_eq!(cipher.decode("123"), "123");
}