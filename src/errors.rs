use std::error::Error;
use std::fmt;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum FileParseError {
    IoError(io::Error),
    ParseError { line: usize, error: ParseIntError },
}

impl fmt::Display for FileParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileParseError::IoError(err) => write!(f, "IO error: {}", err),
            FileParseError::ParseError { line, error } => {
                write!(f, "Parse error at line {}: {}", line, error)
            }
        }
    }
}

impl Error for FileParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FileParseError::IoError(err) => Some(err),
            FileParseError::ParseError { error, .. } => Some(error),
        }
    }
}

impl From<io::Error> for FileParseError {
    fn from(err: io::Error) -> Self {
        FileParseError::IoError(err)
    }
}

pub fn read_numbers_from_file(path: &str) -> Result<Vec<i32>, FileParseError> {
    let content = std::fs::read_to_string(path)?;
    
    content
        .lines()
        .enumerate()
        .map(|(line_num, line)| {
            line.parse()
                .map_err(|e| FileParseError::ParseError {
                    line: line_num + 1,
                    error: e,
                })
        })
        .collect()
}

pub fn error_example() {
    println!("\nError handling example (manual implementation):");
    
    // Try reading from a non-existent file
    match read_numbers_from_file("nonexistent.txt") {
        Ok(numbers) => println!("Numbers: {:?}", numbers),
        Err(e) => println!("Error: {}", e),
    }

    // Create a file with invalid content
    std::fs::write("test.txt", "1\nnot_a_number\n3").unwrap();
    match read_numbers_from_file("test.txt") {
        Ok(numbers) => println!("Numbers: {:?}", numbers),
        Err(e) => println!("Error: {}", e),
    }

    std::fs::write("test.txt", "1\n2\n3").unwrap();
    match read_numbers_from_file("test.txt") {
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
    fn test_valid_file() -> Result<(), Box<dyn Error>> {
        let test_file = "test_numbers.txt";
        let mut file = File::create(test_file)?;
        writeln!(file, "1\n2\n3")?;

        let numbers = read_numbers_from_file(test_file)?;
        assert_eq!(numbers, vec![1, 2, 3]);

        std::fs::remove_file(test_file)?;
        Ok(())
    }

    #[test]
    fn test_invalid_number() {
        let test_file = "test_invalid.txt";
        let mut file = File::create(test_file).unwrap();
        writeln!(file, "1\nnot_a_number\n3").unwrap();

        let result = read_numbers_from_file(test_file);
        assert!(matches!(
            result,
            Err(FileParseError::ParseError { line: 2, .. })
        ));

        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_nonexistent_file() {
        let result = read_numbers_from_file("nonexistent.txt");
        assert!(matches!(result, Err(FileParseError::IoError(_))));
    }
}
