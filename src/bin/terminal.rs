use std::fs::File;

use clap::Parser;
use simple_logger::SimpleLogger;

use transform::{matcher, reader, ui, writer};

#[derive(clap::ArgEnum, Clone, Debug)]
enum LevelFilter {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LevelFilter> for log::LevelFilter {
    fn from(level: LevelFilter) -> Self {
        match level {
            LevelFilter::Off => log::LevelFilter::Off,
            LevelFilter::Error => log::LevelFilter::Error,
            LevelFilter::Warn => log::LevelFilter::Warn,
            LevelFilter::Info => log::LevelFilter::Info,
            LevelFilter::Debug => log::LevelFilter::Debug,
            LevelFilter::Trace => log::LevelFilter::Trace,
        }
    }
}

/// Program to translate transactions into GNUCash format
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    infile: String,
    outfile: String,
    #[clap(long)]
    matchfile: Option<String>,
    #[clap(long)]
    date_col: String,
    #[clap(long, use_delimiter = true, required = true)]
    debtor_col: Vec<String>,
    #[clap(long, use_delimiter = true, required = true)]
    creditor_col: Vec<String>,
    #[clap(long, use_delimiter = true, required = true)]
    desc_col: Vec<String>,
    #[clap(long)]
    debit_col: String,
    #[clap(long)]
    credit_col: String,
    #[clap(long)]
    amount_abs: bool,
    #[clap(arg_enum, long, default_value = "info")]
    log: LevelFilter,
}
fn main() {
    let args = Args::parse();
    log::set_max_level(args.log.into());
    SimpleLogger::new().init().unwrap();

    let read_config = reader::ReadConfig {
        date_field: args.date_col,
        debtor_fields: args.debtor_col,
        creditor_fields: args.creditor_col,
        description_fields: args.desc_col,
        debit_field: args.debit_col,
        credit_field: args.credit_col,
        amount_abs: args.amount_abs,
    };
    let mut transactions =
        reader::validate_and_read(File::open(&args.infile).unwrap(), &read_config).unwrap();
    let mut transformer = match args.matchfile {
        None => matcher::AccountMatcher::new(),
        Some(filename) => matcher::AccountMatcher::from_path(&filename).unwrap(),
    };
    let ui = ui::terminal::UI {};
    for transaction in transactions.iter_mut() {
        transaction.debtor_account = transformer.find_match(&transaction.debtor_account, &ui);
        transaction.creditor_account = transformer.find_match(&transaction.creditor_account, &ui);
    }
    writer::write(File::create(&args.outfile).unwrap(), &transactions).unwrap();
}
