use super::ascii_error::ASCIIError;
use std::cmp;
use std::fs;

#[derive(Clone, Debug)]
pub struct ASCIIPicture {
    pub row_size: u32,
    pub col_size: u32,
    pub text: Vec<Vec<u8>>,
}

impl ASCIIPicture {
    /// # Brief
    /// Create a new `ASCIIPicture` object
    pub fn new(row_size: u32, col_size: u32, text: Vec<Vec<u8>>) -> ASCIIPicture {
        ASCIIPicture {
            row_size: row_size,
            col_size: col_size,
            text: text,
        }
    }

    /// # Brief
    /// Create a new `ASCIIPicture` object by reading from a file.
    /// # Errors
    /// - Throws `ReadFileError` if the file is not accessible.
    pub fn read_from_file(file_name: &str) -> Result<ASCIIPicture, ASCIIError> {
        let text = match fs::read_to_string(file_name) {
            Ok(result) => result.replace("\r\n", "\n"),
            Err(error) => {
                return Err(ASCIIError::ReadFileError(format!(
                    "cannot read file {}, error: {}",
                    file_name, error
                )));
            }
        };

        let mut ascii_picture = ASCIIPicture::new(0, 0, vec![]);

        for ascii_row in text.split("\n") {
            ascii_picture.col_size = cmp::max(ascii_picture.col_size, ascii_row.len() as u32);

            ascii_picture.row_size += 1;

            ascii_picture.text.push(ascii_row.into());
        }

        Ok(ascii_picture)
    }
}

#[cfg(test)]
mod tests {
    use crate::ascii::ascii::ASCIIPicture;

    #[test]
    fn generate_ascii_picture_from_file() {
        let file_name = "/assets/test/test_pic.txt";

        let ascii_picture = ASCIIPicture::read_from_file(file_name).unwrap();
        let correct_ascii_picture = ASCIIPicture::new(
            7,
            15,
            vec![
                vec![
                    b'#', b'#', b'#', b'#', b'#', b'#', b'#', b' ', b'#', b'#', b'#', b'#', b'#',
                    b'#', b'#',
                ],
                vec![
                    b'#', b'#', b'#', b'#', b'#', b'#', b'#', b' ', b'#', b'#', b'#', b'#', b'#',
                    b'#', b'#',
                ],
                vec![
                    b'#', b'#', b'#', b'#', b'#', b'#', b'#', b' ', b'#', b'#', b'#', b'#', b'#',
                    b'#', b'#',
                ],
                vec![
                    b' ', b' ', b' ', b' ', b'W', b'i', b'n', b'd', b'o', b'w', b's',
                ],
                vec![
                    b'#', b'#', b'#', b'#', b'#', b'#', b'#', b' ', b'#', b'#', b'#', b'#', b'#',
                    b'#', b'#',
                ],
                vec![
                    b'#', b'#', b'#', b'#', b'#', b'#', b'#', b' ', b'#', b'#', b'#', b'#', b'#',
                    b'#', b'#',
                ],
                vec![
                    b'#', b'#', b'#', b'#', b'#', b'#', b'#', b' ', b'#', b'#', b'#', b'#', b'#',
                    b'#', b'#',
                ],
            ],
        );

        assert_eq!(ascii_picture.row_size, correct_ascii_picture.row_size);
        assert_eq!(ascii_picture.col_size, correct_ascii_picture.col_size);
        assert_eq!(ascii_picture.text, correct_ascii_picture.text);
    }
}
