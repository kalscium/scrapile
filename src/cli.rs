use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(index=1, help="The path of the program source code")]
    pub input_file: String,
    #[arg(index=2, help="The desired output path of the compiled scratch binary")]
    pub output_file: String,
}
