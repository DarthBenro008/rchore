use console::{style, Term};

pub fn print_error(action: &str, error: &std::boxed::Box<dyn std::error::Error>) {
    println!(
        "{} {}\n{}: {}",
        style("Uh oh, somthing went wrong while")
            .for_stderr()
            .bold()
            .red(),
        style(format!("{} {}", action, ":("))
            .for_stderr()
            .bold()
            .red(),
        style("Reason").for_stderr().red().underlined(),
        style(error).for_stderr().red()
    );
}

pub fn print_success(action: String) {
    println!(
        "{}\n{}",
        style("Yay! :)").for_stdout().bold().green(),
        style(action).for_stdout().green(),
    )
}

pub fn print_ok(data: String) {
    println!("{}", style(data).for_stdout().green())
}

pub fn print_warning(data: String) {
    println!(
        "{}{}",
        style("[Warning]: ").for_stdout().yellow(),
        style(data).for_stdout().yellow()
    )
}

pub fn print_red(action: &str) {
    println!(
        "{} {}",
        style("Uh oh, somthing went wrong while")
            .for_stderr()
            .bold()
            .red(),
        style(format!("{} {}", action, ":\")"))
            .for_stderr()
            .bold()
            .red(),
    )
}

pub fn url_print(url: reqwest::Url) {
    println!(
        "{} \n{} {}",
        style("Open the following link in your browser to authenticate yourself:")
            .yellow()
            .bold(),
        style("[LINK]:").for_stdout().yellow(),
        style(url).for_stdout().white(),
    )
}

pub fn force_write(action: String) -> anyhow::Result<()> {
    let term = Term::stdout();
    term.clear_last_lines(3)?;
    print_ok(action);
    Ok(())
}
