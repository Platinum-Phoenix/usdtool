#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};
use std::str::FromStr;

enum Coin {
    Penny(u32),
    Nickel(u32),
    Dime(u32),
    Quarter(u32),
}

fn sum_coins(coins: [Coin; 4]) -> u32 {
    let mut sum: u32 = 0;
    for coin in coins.iter() {
        sum += to_cents(coin);
    }
    sum
}
fn to_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny(x) => 1 * x,
        Coin::Nickel(x) => 5 * x,
        Coin::Dime(x) => 10 * x,
        Coin::Quarter(x) => 25 * x,
    }
}

fn main() {
    let matches = App::new("usdtool")
        .about("Useful tool for the US Dollar.")
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("sum")
                .about("Outputs the total dollar value from pennies, nickels, dimes, and quarters.")
                .arg(
                    Arg::with_name("pennies")
                        .short("p")
                        .long("pennies")
                        .help("The amount of pennies.")
                        .takes_value(true)
                        .required(true)
                        .default_value("0"),
                )
                .arg(
                    Arg::with_name("nickels")
                        .short("n")
                        .long("nickels")
                        .help("The amount of nickels.")
                        .takes_value(true)
                        .required(true)
                        .default_value("0"),
                )
                .arg(
                    Arg::with_name("dimes")
                        .short("d")
                        .long("dimes")
                        .help("The amount of dimes.")
                        .takes_value(true)
                        .required(true)
                        .default_value("0"),
                )
                .arg(
                    Arg::with_name("quarters")
                        .short("q")
                        .long("quarters")
                        .help("The amount of quarters.")
                        .takes_value(true)
                        .required(true)
                        .default_value("0"),
                ),
        )
        .get_matches();

    // The most common way to handle subcommands is via a combined approach using
    // `ArgMatches::subcommand` which returns a tuple of both the name and matches
    match matches.subcommand() {
        ("sum", Some(sum_matches)) => {
            // Now we have a reference to sum's matches
            let quarters = u32::from_str(sum_matches.value_of("quarters").unwrap()).unwrap();
            let dimes = u32::from_str(sum_matches.value_of("dimes").unwrap()).unwrap();
            let nickels = u32::from_str(sum_matches.value_of("nickels").unwrap()).unwrap();
            let pennies = u32::from_str(sum_matches.value_of("pennies").unwrap()).unwrap();

            println!(
                "Summing: {:?} quarters, {:?}, dimes, {:?} nickels, and {:?} pennies.",
                quarters, dimes, nickels, pennies,
            );

            let cents = sum_coins([
                Coin::Quarter(quarters),
                Coin::Dime(dimes),
                Coin::Nickel(nickels),
                Coin::Penny(pennies),
            ]);

            println!("The sum is {:?} cents.", cents);
            let dollars: f32 = cents as f32 / 100_f32;
            println!("Or {:?} dollars", dollars);
        }
        ("", None) => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}
