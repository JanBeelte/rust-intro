use std::io;
use std::num::ParseIntError;
use thiserror::Error;
use std::error;


// New error type using thiserror
#[derive(Error, Debug)]
pub enum FileParseError2 {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    
    #[error("Parse error at line {line}: {error}")]
    ParseError {
        line: usize,
        #[source]
        error: ParseIntError,
    }
}

// New function using FileParseError2
pub fn read_numbers_from_file2(path: &str) -> Result<Vec<i32>, FileParseError2> {
    let content = std::fs::read_to_string(path)?;
    
    content
        .lines()
        .enumerate()
        .map(|(line_num, line)| {
            line.parse()
                .map_err(|error| FileParseError2::ParseError {
                    line: line_num + 1,
                    error,
                })
        })
        .collect()
}

pub fn error_example() {
    println!("\nError handling example (thiserror implementation):");
    
    // Try with thiserror version
    match read_numbers_from_file2("nonexistent.txt") {
        Ok(numbers) => println!("Numbers: {:?}", numbers),
        Err(e) => println!("Error: {}", e),
    }

    // Create a file with invalid content
    std::fs::write("test.txt", "1\nnot_a_number\n3").unwrap();
    match read_numbers_from_file2("test.txt") {
        Ok(numbers) => println!("Numbers: {:?}", numbers),
        Err(e) => println!("Error: {}", e),
    }

    // Create a file with valid content
    std::fs::write("test.txt", "1\n2\n3").unwrap();
    match read_numbers_from_file2("test.txt") {
        Ok(numbers) => println!("Numbers: {:?}", numbers),
        Err(e) => println!("Error: {}", e),
    }

    // Clean up
    let _ = std::fs::remove_file("test.txt");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_valid_file2() -> Result<(), Box<dyn error::Error>> {
        let test_file = "test_numbers2.txt";
        let mut file = File::create(test_file)?;
        writeln!(file, "1\n2\n3")?;

        let numbers = read_numbers_from_file2(test_file)?;
        assert_eq!(numbers, vec![1, 2, 3]);

        std::fs::remove_file(test_file)?;
        Ok(())
    }

    #[test]
    fn test_invalid_number2() {
        let test_file = "test_invalid2.txt";
        let mut file = File::create(test_file).unwrap();
        writeln!(file, "1\nnot_a_number\n3").unwrap();

        let result = read_numbers_from_file2(test_file);
        assert!(matches!(
            result,
            Err(FileParseError2::ParseError { line: 2, .. })
        ));

        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_nonexistent_file2() {
        let result = read_numbers_from_file2("nonexistent2.txt");
        assert!(matches!(result, Err(FileParseError2::IoError(_))));
    }
}