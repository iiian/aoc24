mod dec_01;
mod helper;

macro_rules! run_puzzles {
    ($mod_name:ident) => {{
        println!(
            "{}, Puzzle #1 -- RESULT: {}",
            stringify!($mod_name),
            $mod_name::puzzle1()?
        );
        println!(
            "{}, Puzzle #2 -- RESULT: {}",
            stringify!($mod_name),
            $mod_name::puzzle2()?
        );
    }};
}

fn main() -> Result<(), std::io::Error> {
    run_puzzles!(dec_01);

    Ok(())
}
