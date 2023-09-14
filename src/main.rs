mod scanner;

fn main() {
    let string_to_parse = "**";
    let res = string(string_to_parse);

    println!("{}, RESULT: {}", string_to_parse, res);
}

fn string(string: &str) -> bool {
    let mut scan = scanner::Scanner::new(string);
    loop {
        if !unit(&mut scan) {
            break;
        }
    }
    scan.cursor() > 0 && scan.is_done()
}

fn unit(scanner: &mut scanner::Scanner) -> bool {
    return scanner.take(&'*');
}

fn value(scanner: &mut scanner::Scanner) -> Option<u8> {
    return scanner.transform(|character| match character {
        '$' => Some(1),
        '#' => Some(2),
        _ => None,
    });
}
