use std::error::Error;
use std::fs;
use std::path::Path;
use std::str::FromStr;

/// Parses a file where each line is automatically converted to type `T`.
///
/// This function reads a file and parses each line using the type's `FromStr` implementation.
/// It's useful when your input file has one value per line that can be parsed into a specific type.
///
/// # Type Parameters
///
/// * `T` - The target type that implements `FromStr`. Common types include `i32`, `f64`, `String`, etc.
/// * `P` - Any path-like type (e.g., `&str`, `String`, `PathBuf`)
///
/// # Arguments
///
/// * `path` - Path to the input file
///
/// # Returns
///
/// * `Ok(Vec<T>)` - Vector of successfully parsed values
/// * `Err` - If the file cannot be read or any line fails to parse
///
/// # Examples
///
/// ```no_run
/// use aoclib::parse_lines;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Parse a file of integers
/// let numbers: Vec<i32> = parse_lines("numbers.txt")?;
///
/// // Parse a file of floating point numbers
/// let values: Vec<f64> = parse_lines("data.txt")?;
///
/// // Parse a file of strings
/// let lines: Vec<String> = parse_lines("text.txt")?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// * The file cannot be read
/// * Any line in the file cannot be parsed into type `T`
pub fn parse_lines<T, P>(path: P) -> Result<Vec<T>, Box<dyn Error>>
where
    T: FromStr,
    T::Err: std::error::Error + 'static,
    P: AsRef<Path>,
{
    let content = fs::read_to_string(path)?;
    content
        .lines()
        .map(|line| line.parse::<T>().map_err(|e| e.into()))
        .collect()
}

/// Parses a file using a custom parser function for each line.
///
/// This function provides maximum flexibility by allowing you to define exactly how each
/// line should be parsed. Use this when lines contain structured data that needs custom parsing.
///
/// # Type Parameters
///
/// * `T` - The target type to parse each line into
/// * `P` - Any path-like type (e.g., `&str`, `String`, `PathBuf`)
/// * `F` - A function that takes a string slice and returns `Result<T, Box<dyn Error>>`
///
/// # Arguments
///
/// * `path` - Path to the input file
/// * `parser` - Function that parses a single line into type `T`
///
/// # Returns
///
/// * `Ok(Vec<T>)` - Vector of successfully parsed values
/// * `Err` - If the file cannot be read or any line fails to parse
///
/// # Examples
///
/// ```no_run
/// use aoclib::parse_lines_with;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Parse CSV-like data
/// let data = parse_lines_with("data.csv", |line| {
///     let parts: Vec<&str> = line.split(',').collect();
///     let name = parts[0].to_string();
///     let age = parts[1].parse::<u32>()?;
///     Ok((name, age))
/// })?;
///
/// // Parse complex structured data
/// let coords = parse_lines_with("points.txt", |line| {
///     let nums: Vec<i32> = line
///         .split_whitespace()
///         .map(|n| n.parse())
///         .collect::<Result<_, _>>()?;
///     Ok((nums[0], nums[1]))
/// })?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// * The file cannot be read
/// * The parser function returns an error for any line
pub fn parse_lines_with<T, P, F>(path: P, parser: F) -> Result<Vec<T>, Box<dyn Error>>
where
    P: AsRef<Path>,
    F: Fn(&str) -> Result<T, Box<dyn Error>>,
{
    let content = fs::read_to_string(path)?;
    content.lines().map(parser).collect()
}

/// Parses an entire file using a custom parser function.
///
/// Unlike `parse_lines_with`, this function passes the entire file content as a single string
/// to your parser. Use this when you need to handle the file as a whole, such as parsing
/// sections separated by blank lines or processing multi-line records.
///
/// # Type Parameters
///
/// * `T` - The target type to parse the file content into
/// * `P` - Any path-like type (e.g., `&str`, `String`, `PathBuf`)
/// * `F` - A function that takes the entire file content and returns `Result<T, Box<dyn Error>>`
///
/// # Arguments
///
/// * `path` - Path to the input file
/// * `parser` - Function that parses the entire file content into type `T`
///
/// # Returns
///
/// * `Ok(T)` - Successfully parsed result
/// * `Err` - If the file cannot be read or parsing fails
///
/// # Examples
///
/// ```no_run
/// use aoclib::parse_with;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Parse sections separated by blank lines
/// let sections: Vec<String> = parse_with("input.txt", |content| {
///     let parts: Vec<String> = content
///         .split("\n\n")
///         .map(|s| s.to_string())
///         .collect();
///     Ok(parts)
/// })?;
///
/// // Parse a grid/matrix
/// let grid: Vec<Vec<char>> = parse_with("grid.txt", |content| {
///     let rows: Vec<Vec<char>> = content
///         .lines()
///         .map(|line| line.chars().collect())
///         .collect();
///     Ok(rows)
/// })?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// * The file cannot be read
/// * The parser function returns an error
pub fn parse_with<T, P, F>(path: P, parser: F) -> Result<T, Box<dyn Error>>
where
    P: AsRef<Path>,
    F: Fn(&str) -> Result<T, Box<dyn Error>>,
{
    let content = fs::read_to_string(path)?;
    parser(&content)
}

/// Reads a file and returns its contents as a raw string.
///
/// This is the simplest function - it just reads the entire file content without any parsing.
/// Use this when you want to handle the parsing yourself or just need the raw file content.
///
/// # Arguments
///
/// * `path` - Path to the input file
///
/// # Returns
///
/// * `Ok(String)` - The file contents
/// * `Err` - If the file cannot be read
///
/// # Examples
///
/// ```no_run
/// use aoclib::read_input;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Read file as raw string
/// let content = read_input("input.txt")?;
///
/// // Then process it however you want
/// for line in content.lines() {
///     println!("{}", line);
/// }
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// * The file does not exist
/// * The file cannot be read (permissions, etc.)
/// * The file contains invalid UTF-8
pub fn read_input<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    fn create_test_file(name: &str, content: &str) -> String {
        let path = format!("test_{}.txt", name);
        let mut file = File::create(&path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        path
    }

    fn clean_up_test_file(path: &str) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_parse_lines_integers() {
        let path = create_test_file("integers", "1\n2\n3\n4\n5");

        let result: Result<Vec<i32>, _> = parse_lines(&path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 2, 3, 4, 5]);

        clean_up_test_file(&path);
    }

    #[test]
    fn test_parse_lines_strings() {
        let path = create_test_file("strings", "hello\nworld\ntest");

        let result: Result<Vec<String>, _> = parse_lines(&path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["hello", "world", "test"]);

        clean_up_test_file(&path);
    }

    #[test]
    fn test_parse_lines_floats() {
        let path = create_test_file("floats", "1.5\n2.7\n3.14");

        let result: Result<Vec<f64>, _> = parse_lines(&path);
        assert!(result.is_ok());
        let values = result.unwrap();
        assert_eq!(values.len(), 3);
        assert!((values[0] - 1.5).abs() < 0.001);
        assert!((values[1] - 2.7).abs() < 0.001);
        assert!((values[2] - 3.14).abs() < 0.001);

        clean_up_test_file(&path);
    }

    #[test]
    fn test_parse_lines_with_custom_parser() {
        let path = create_test_file("csv", "apple,5\nbanana,3\norange,7");

        let result = parse_lines_with(&path, |line| {
            let parts: Vec<&str> = line.split(',').collect();
            let name = parts[0].to_string();
            let count = parts[1].parse::<i32>()?;
            Ok((name, count))
        });

        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.len(), 3);
        assert_eq!(data[0], ("apple".to_string(), 5));
        assert_eq!(data[1], ("banana".to_string(), 3));
        assert_eq!(data[2], ("orange".to_string(), 7));

        clean_up_test_file(&path);
    }

    #[test]
    fn test_parse_with_sections() {
        let path = create_test_file("sections", "section1\nline1\nline2\n\nsection2\nline3");

        let result = parse_with(&path, |content| {
            let sections: Vec<String> = content.split("\n\n").map(|s| s.to_string()).collect();
            Ok(sections)
        });

        assert!(result.is_ok());
        let sections = result.unwrap();
        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0], "section1\nline1\nline2");
        assert_eq!(sections[1], "section2\nline3");

        clean_up_test_file(&path);
    }

    #[test]
    fn test_read_input() {
        let content = "Hello, World!\nThis is a test.";
        let path = create_test_file("raw", content);

        let result: Result<String, _> = read_input(&path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content);

        clean_up_test_file(&path);
    }

    #[test]
    fn test_parse_lines_empty_file() {
        let path = create_test_file("empty", "");

        let result: Result<Vec<String>, _> = parse_lines(&path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Vec::<String>::new());

        clean_up_test_file(&path);
    }

    #[test]
    fn test_parse_lines_invalid_format() {
        let path = create_test_file("invalid", "1\n2\nNaN\n4");

        let result: Result<Vec<i32>, _> = parse_lines(&path);
        assert!(result.is_err());
        clean_up_test_file(&path);
    }

    #[test]
    fn test_nonexistent_file() {
        let result: Result<Vec<String>, _> = parse_lines("nonexistent_file.txt");
        assert!(result.is_err());
    }
}
