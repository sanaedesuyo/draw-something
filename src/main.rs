use draw_something::{ascii_hook::hook::{hooks::SingleFileHook, IASCIIHook}, display::display::{Displayer, IDisplay}};

fn main() {
    let single_hook = SingleFileHook;

    let source = "assets/test/test_pic.txt";
    let ascii_pic = match single_hook.convert(source) {
        Ok(pic) => pic,
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    };

    let displayer = Displayer;
    
    println!("{}", displayer.display(ascii_pic));
}