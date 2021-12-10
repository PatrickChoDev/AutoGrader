use walkdir::WalkDir;

pub struct Test {
    id: String,
    pub cases: Option<Vec<TestCase>>,
    score: u32,
}

pub struct TestCase {
    id: String,
    pub input: String,
    pub output: Option<String>,
    pub score: Option<u32>,
}

pub fn parse_ext(filename: &str) -> Result<Vec<&str>, &str> {
    let sep: Vec<&str> = filename.split_inclusive(".").collect();
    if sep.len() > 0 {
        return Ok(sep[1..].to_vec());
    } else {
        return Err("No file extension found.");
    }
}

pub fn find_testcases(dirname: &str) -> Option<Vec<String>> {
    let mut input_streams: Vec<String> = vec![];
    for e in WalkDir::new(dirname).into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            let filepath = e.path().display().to_string();
            if parse_ext(&filepath)
                .ok()
                .unwrap_or([""].to_vec())
                .last()
                .unwrap()
                == &"txt"
            { input_streams.push(filepath)}
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
fn parse_file_extension() {
    assert_eq!(parse_ext("tests/config.json").unwrap(), ["json"].to_vec());
    assert_eq!(
        parse_ext("tests/sum.test.json").unwrap(),
        ["test.", "json"].to_vec()
    );
}

#[test]
fn find_input_cases() {
    assert!(find_testcases("tests/sum").is_some());
}
