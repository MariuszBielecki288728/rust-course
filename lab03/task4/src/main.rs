fn main() {
    println!("Hello, world!");
}

fn zoom(n: i32) -> String {
    let mut vec = Vec::new();
    for i in (0..n) {
        for j in (0..n) {
            if ((i - n / 2) % 2 == 0) &&  n - i ((j - n / 2) % 2 == 0) {
                vec.push("■")
            } else {
                vec.push("□")
            }
        }
        vec.push("\n")
    };
    vec.connect("")
}

#[test]
fn basic_test_1() {
  assert_eq!(zoom(1), "■");
}

#[test]
fn basic_test_2() {
  assert_eq!(zoom(3), "\
□□□
□■□
□□□"
  );
}

#[test]
fn basic_test_3() {
  assert_eq!(zoom(5), "\
■■■■■
■□□□■
■□■□■
■□□□■
■■■■■"
  );
}

#[test]
fn basic_test_4() {
  assert_eq!(zoom(7), "\
□□□□□□□
□■■■■■□
□■□□□■□
□■□■□■□
□■□□□■□
□■■■■■□
□□□□□□□"
  );
}

#[test]
fn basic_test_5() {
  assert_eq!(zoom(9), "\
■■■■■■■■■
■□□□□□□□■
■□■■■■■□■
■□■□□□■□■
■□■□■□■□■
■□■□□□■□■
■□■■■■■□■
■□□□□□□□■
■■■■■■■■■"
  );
}