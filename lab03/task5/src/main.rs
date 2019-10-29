
fn main() {
    println!("Hello, world!");
}

fn last_digit(str1: &str, str2: &str) -> i32 {
  if str2 == "0" { return 1; }
  let last_of_str1 = str1.chars().last().unwrap().to_digit(10).unwrap();
  let last_two_of_str2 = match str2.len() {
      1 => str2.chars().nth(0).unwrap().to_digit(10).unwrap(),
      n => str2[(n-2)..n].to_string().parse().unwrap(),
  };

  (last_of_str1.pow(match last_two_of_str2 % 4 {0 => 4, n => n}) % 10) as i32
}

#[test]
fn returns_expected1() {
  assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376","2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
}

#[test]
fn returns_expected2() {
  assert_eq!(last_digit("3715290469715693021198967285016729344580685479654510946723", "68819615221552997273737174557165657483427362207517952651"), 7);
}

#[test]
fn returns_expected3() {
  assert_eq!(last_digit("4", "1"), 4);
}

#[test]
fn returns_expected4() {
  assert_eq!(last_digit("144", "2"), 6);
}

#[test]
fn returns_expected5() {
  assert_eq!(last_digit("9", "7"), 9);
}

#[test]
fn returns_expected6() {
  assert_eq!(last_digit("10","10000000000"), 0);
}

#[test]
fn returns_expected7() {
  assert_eq!(last_digit("1","1"), 1);
}

#[test]
fn returns_expected8() {
  assert_eq!(last_digit("2","0"), 1);
}


#[test]
fn returns_expected9() {
  assert_eq!(last_digit("44031869921499681305272023313238731176974798344490351749962700938078", "1423438398351559910572"
), 6);
}

#[test]
fn returns_expected10() {
  assert_eq!(last_digit("1","3"), 1);
}