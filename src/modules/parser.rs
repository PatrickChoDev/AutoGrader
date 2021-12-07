use walkdir::WalkDir;

pub fn parse_ext(filename: &str) -> Result<&str, &str> {
    match filename.split_inclusive(".").last() {
        Some(ext) => Ok(ext),
        None => Err("No file extension found in file"),
    }
}

pub fn load_input(filename: &str) -> Option<String> {
    let buffer = std::fs::read_to_string(filename);
    match buffer {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}

pub fn find_input(dirname: &str) -> Option<Vec<String>> {
    let mut input_streams: Vec<String> = vec![];
    for e in WalkDir::new(dirname).into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            let filepath = e.path().display().to_string();
            if parse_ext(&filepath).ok() == Some("txt") {
                match load_input(&filepath) {
                    Some(d) => input_streams.push(d),
                    None => (),
                }
            }
        }
    }
    if input_streams.len() > 0 {
        return Some(input_streams);
    } else {
        return None;
    }
}

#[cfg(test)]
#[test]
fn read_input_file() {
    assert!(load_input("tests/sum/sum1.txt").is_some());
}

#[test]
fn find_input_cases() {
    assert!(find_input("tests/sum").is_some());
}
