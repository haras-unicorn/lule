// #![deny(
//   unsafe_code,
//   // reason = "Let's just not do it"
// )]
// #![deny(
//   clippy::unwrap_used,
//   clippy::expect_used,
//   clippy::panic,
//   // reason = "We have to handle errors properly"
// )]

mod cli;
mod fun;
mod gen;
mod scheme;
mod show;
mod var;

use scheme::Scheme;
use std::env;

fn main() {
    let mut scheme = Scheme::init();

    let show_logo = env::args().len() <= 1;
    let current_exe_path = env::current_exe().unwrap();
    let current_exe_dir = current_exe_path.parent().unwrap();
    let logo_path = current_exe_dir.join("assets").join("logo.txt");
    let logo = if show_logo {
        std::fs::read_to_string(logo_path).unwrap_or(String::new())
    } else {
        String::new()
    };

    let app = cli::build_cli(logo.as_str()).get_matches();
    // var::concatinate(&app, &mut scheme);

    if let Some(subcommand) = app.subcommand_name() {
        match subcommand {
            "colors" => cli::colors::run(&app, &mut scheme),
            "create" => cli::create::run(&app, &mut scheme),
            "config" => cli::config::run(&app, &mut scheme),
            "test" => cli::test::run(&app, &mut scheme),
            _ => Ok(()),
        }
        .ok();
    }
}
