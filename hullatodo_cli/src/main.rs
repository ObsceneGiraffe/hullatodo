extern crate clap;

use clap::App;

fn main() {
    let app = App::new("hulla")
        .version("0.1")
        .author("Hullatodo. <hullatodo@heylin.nl>")
        .about("Hullatodo CLI");

    let _matches = app.get_matches();
}

