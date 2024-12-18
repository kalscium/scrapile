use std::fs;
use color_eyre::eyre::Context;
use logos::Logos;
use clap::Parser;
use scrapile::{cli::Cli, lang::{error::Reportable, parser, targets, token::Token, typed}, scratch::add_console};

/// Go through all the errors and report them
fn throw_lang_error<T>(src: &str, src_id: &str, errors: &[impl Reportable]) -> T {
    for error in errors {
        error.report(src_id, src);
    }

    std::process::exit(1);
}

fn main() {
    // setup color-eyre
    color_eyre::install().unwrap();

    // parse the cli
    let cli = Cli::parse();

    // read the input program file
    let src = fs::read_to_string(&cli.input_file)
        .wrap_err_with(|| format!("while reading contents of source-code file `{}`", cli.input_file))
        .unwrap();
    
    // lex, parse, type-check and assemble the project
    let mut tokens = Token::lexer(&src).spanned();
    let roots = match parser::root::parse_root(&mut tokens) {
        Ok(ok) => ok,
        Err(err) => throw_lang_error(&src, &cli.input_file, &err),
    };
    let project = match typed::root::wrap_root(&roots) {
        Ok(ok) => ok,
        Err(err) => throw_lang_error(&src, &cli.input_file, &[err]),
    };
    let assembly = targets::scratch::translate(project);
    let json = add_console("console", scrapile::scratch::assemble(assembly));

    // write the assembled project to the output path
    scrapile::scratch::write_to_zip(&cli.output_file, json)
        .with_context(|| format!("while writing compiled scratch binary to output path `{}`", cli.output_file))
        .unwrap();
}
