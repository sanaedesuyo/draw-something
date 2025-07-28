pub trait IASCIIHook<'a> {
    type Source;
    type Result;
    fn convert(self, source: Self::Source) -> Self::Result;
}

pub mod hooks {
    use crate::{ascii::{ascii::ASCIIPicture, ascii_error::ASCIIError}, ascii_hook::hook::IASCIIHook};

    pub struct SingleFileHook;
    impl<'a> IASCIIHook<'a> for SingleFileHook {
        type Source = &'a str;
        type Result = Result<ASCIIPicture, ASCIIError>;
    
        fn convert(self, source: Self::Source) -> Self::Result {
            ASCIIPicture::read_from_file(source)
        }
    }

    pub struct MultiFileHook;
    impl<'a> IASCIIHook<'a> for MultiFileHook {
        type Source = Vec<&'a str>;
        type Result = Result<Vec<ASCIIPicture>, ASCIIError>;

        fn convert(self, source: Self::Source) -> Self::Result {
            let mut ascii_pics = Vec::new();

            for filename in source {
                match ASCIIPicture::read_from_file(filename) {
                    Ok(ascii_pic) => ascii_pics.push(ascii_pic),
                    Err(error) => return Err(error)
                }
            }

            Ok(ascii_pics)
        }
    }
}

