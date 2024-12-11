use day_11::part1_recursive_cached::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input2.txt");
    let result = process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}