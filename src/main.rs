use core::f64;
use std::{ io, num::ParseFloatError };
use colorz::{ xterm, Colorize, Effect, Style };

fn fahrenheit_to_celsius(f: f64) -> f64 {
    ((f - 32.0) * 5.0) / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0) / 5.0 + 32.0
}

fn get_input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

fn parse_float(input: &str) -> Result<f64, ParseFloatError> {
    input.trim().parse::<f64>()
}

const INSTRUCTIONS_TITLE_STYLE: Style = Style::new()
    .fg(xterm::Yellow)
    .effects_array([Effect::Bold])
    .const_into_runtime_style();

const INSTRUCTIONS_STYLE: Style = Style::new().fg(xterm::Cyan).const_into_runtime_style();

const ERROR_STYLE: Style = Style::new()
    .fg(xterm::Red)
    .effects_array([Effect::Blink, Effect::Bold])
    .const_into_runtime_style();

fn main() -> io::Result<()> {
    Ok(loop {
        println!("{}", "Please select the conversion type:".style_with(INSTRUCTIONS_TITLE_STYLE));
        println!("{}", "1. Fahrenheit to Celsius".style_with(INSTRUCTIONS_STYLE));
        println!("{}", "2. Celsius to Fahrenheit".style_with(INSTRUCTIONS_STYLE));
        println!("{}", "3. Exit".bright_white());

        let input = get_input()?;

        match input.trim() {
            "1" => {
                println!(
                    "{}",
                    "Enter the temperature in Fahrenheit:".style_with(INSTRUCTIONS_STYLE)
                );
                let f = get_input()?;
                match parse_float(&f) {
                    Ok(f) => println!("{}°F = {}°C", f, fahrenheit_to_celsius(f)),
                    Err(_parse_float_error) => {
                        println!(
                            "{}",
                            "Invalid format, please use a '.' as decimal separator".style_with(
                                ERROR_STYLE
                            )
                        );
                        continue;
                    }
                };
            }
            "2" => {
                println!("{}", "Enter the temperature in Celsius:".style_with(INSTRUCTIONS_STYLE));
                let c = get_input()?;
                match parse_float(&c) {
                    Ok(c) => println!("{}°C = {}°F", c, celsius_to_fahrenheit(c)),
                    Err(_parse_float_error) => {
                        println!(
                            "{}",
                            "Invalid format, please use a '.' as decimal separator".style_with(
                                ERROR_STYLE
                            )
                        );
                        continue;
                    }
                };
            }
            "3" => {
                println!("{}", "Ciao!".style_with(INSTRUCTIONS_STYLE));
                break;
            }
            _ => {
                println!("{}", "Invalid input".style_with(ERROR_STYLE));
            }
        }
    })
}
