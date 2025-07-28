use crate::ascii::ascii::ASCIIPicture;

pub trait IDisplay {
    type Displayable: std::fmt::Display;

    fn display(&self, ascii_pic: ASCIIPicture) -> Self::Displayable;
}

pub struct Displayer;
impl IDisplay for Displayer {
    type Displayable = String;

    fn display(&self, ascii_pic: ASCIIPicture) -> Self::Displayable {
        let mut text = String::new();
        for row in ascii_pic.text {
            for ch in row {
                text.push(ch as char);
            }

            text.push('\n')
        }

        text
    }
}