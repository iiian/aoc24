mod dec_01;
mod dec_02;
mod dec_03;
mod dec_04;
mod dec_05;

macro_rules! run_puzzles {
    ($mod_name:ident) => {{
        println!("{}", stringify!($mod_name));
        println!("\tPuzzle #1 -- RESULT: {}", $mod_name::puzzle1()?);
        println!("\tPuzzle #2 -- RESULT: {}", $mod_name::puzzle2()?);
    }};
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // run_puzzles!(dec_01);
    // run_puzzles!(dec_02);
    // run_puzzles!(dec_03);
    // run_puzzles!(dec_04);
    run_puzzles!(dec_05);

    Ok(())
}
