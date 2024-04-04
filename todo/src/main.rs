use clap::Parser;

fn main() {
    let cli = Cli::parse();
    if let Some(task) = cli.task.as_deref() {
        println!("{}", task);
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    task: Option<String>,
}
