fn generate_bcdef_from_abcde(input: Vec<char>) -> Vec<char> {
  //if char is z or Z, return a or A
    input.iter().map(|&c| {
        if c == 'z' || c == 'Z' {
            return 'a';
        }
        (c as u8 + 1) as char
    }).collect()
}

fn main() {
    let abcde = vec!['a', 'b', 'c', 'd', 'e'];
    let result = generate_bcdef_from_abcde(abcde);

    println!("{:?}", result);
}
