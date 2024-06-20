use clap::Parser;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

#[derive(Parser)]
pub struct Cli {
    /// The spread of the two strikes
    #[arg(long, short = 's')]
    spread: Decimal,
    /// The number of days till expiration
    #[arg(long, short = 'd')]
    days: Decimal,
    /// The desired effective yield
    #[arg(long, short = 'r')]
    rate: Decimal,
}
fn main() {
    let args = Cli::parse();
    let yr_over_days = Decimal::from_u64(365_u64).unwrap() / args.days;
    let price: Decimal = args.spread / (Decimal::ONE + (args.rate / yr_over_days));
    println!("Price: {}", price);
}
