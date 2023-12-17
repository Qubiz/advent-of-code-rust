use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Mathijs de Groot", name = "advent-of-code", version = "0.1.0", about = "Advent of Code CLI")]
pub struct Arguments {
    #[arg(short, long, default_value = "2023")]
    pub(crate) year: u16,

    #[arg(short, long, default_value = "1")]
    pub(crate) day: u8,
}
