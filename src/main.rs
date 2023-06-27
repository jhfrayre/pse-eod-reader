use lazy_static::lazy_static;
use regex::Regex;
use serde::ser::{
    Serialize,
    Serializer,
    SerializeStruct,
};
use std::{
    env,
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
    process,
};

const COLUMNS: usize = 11;

struct Stock {
    company: String,
    symbol: String,
    bid: String,
    ask: String,
    open: String,
    high: String,
    low: String,
    close: String,
    volume: String,
    value: String,
    net_foreign_trades: String
}

impl Serialize for Stock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Stock", COLUMNS)?;
        state.serialize_field("company", &self.company)?;
        state.serialize_field("symbol", &self.symbol)?;
        state.serialize_field("bid", &self.bid)?;
        state.serialize_field("ask", &self.ask)?;
        state.serialize_field("open", &self.open)?;
        state.serialize_field("high", &self.high)?;
        state.serialize_field("low", &self.low)?;
        state.serialize_field("close", &self.close)?;
        state.serialize_field("volume", &self.volume)?;
        state.serialize_field("value", &self.value)?;
        state.serialize_field("net_foreign_trades", &self.net_foreign_trades)?;

        state.end()
    }
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

fn example() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let reg: Regex = Regex::new(r"((\w+(\s[\w])*)+(\s{2,})\w+(\s{2,})\d{1,3}(,\d{3})*(.\d*)*)").unwrap();
    let mut writer = csv::Writer::from_writer(io::stdout());

    let filename: &str = &args[1];
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(quote) = line {
                if reg.is_match(&quote) {
                    let stock: Stock = parse_line(quote.to_string());
                    writer.serialize(stock)?;
                }
            }
        }
    }
    writer.flush()?;
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: String) -> Stock {
    lazy_static! {
        static ref SEPARATOR: Regex = Regex::new(r"\s{2,}").unwrap();
    }
    let result: Vec<&str> = SEPARATOR.splitn(&line, COLUMNS).collect();
    let stock = Stock {
        company: result[0].to_string(),
        symbol: result[1].to_string(),
        bid: result[2].to_string(),
        ask: result[3].to_string(),
        open: result[4].to_string(),
        high: result[5].to_string(),
        low: result[6].to_string(),
        close: result[7].to_string(),
        volume: result[8].to_string(),
        value: result[9].to_string(),
        net_foreign_trades: result[10].to_string()
    };
    stock
}
