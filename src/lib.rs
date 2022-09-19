pub fn interpret(input: &str, mut writer: impl std::io::Write) {
    let mut acc: i64 = 0;

    let char_vec: Vec<char> = input.chars().collect();
    for c in char_vec {
        match c {
            'i' => acc += 1,
            'd' => acc -= 1,
            's' => acc *= acc,
            'o' => writeln!(writer, "{}", acc).unwrap(),
            '\n' => continue,
            _ => writeln!(writer, "").unwrap(),
        }
        acc = if acc == 256 || acc < 0 { 0 } else { acc }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let mut result = Vec::new();
        interpret("iissso", &mut result);
        assert_eq!(result[0] as char, '0');
    }
    
    #[test]
    fn second() {
        let mut result = Vec::new();
        interpret("diissisdo", &mut result);
        assert_eq!(result[0] as char, '2');
        assert_eq!(result[1] as char, '8');
        assert_eq!(result[2] as char, '8');
    }
    #[test]
    fn third() {
        let mut result = Vec::new();
        interpret("iissisdddddddddddddddddddddddddddddddddo", &mut result);
        assert_eq!(result[0] as char, '0');
    }
}
