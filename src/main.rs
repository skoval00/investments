#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate serde_derive;

#[macro_use] mod core;
#[macro_use] mod types;
mod analyse;
mod broker_statement;
mod brokers;
mod config;
mod currency;
mod db;
mod formatting;
mod init;
mod portfolio;
mod quotes;
mod regulations;
mod tax_statement;
mod util;

use std::process;

use log::error;

use crate::config::Config;
use crate::core::EmptyResult;
use crate::init::{Action, initialize};

fn main() {
    let (action, config) = initialize();

    if let Err(e) = run(action, config) {
        error!("{}.", e);
        process::exit(1);
    }
}

fn run(action: Action, config: Config) -> EmptyResult {
    match action {
        Action::Analyse(name) => analyse::analyse(&config, &name)?,

        Action::Sync(name) => portfolio::sync(&config, &name)?,
        Action::Buy(name, shares, symbol, cash_assets) =>
            portfolio::buy(&config, &name, shares, &symbol, cash_assets)?,
        Action::Sell(name, shares, symbol, cash_assets) =>
            portfolio::sell(&config, &name, shares, &symbol, cash_assets)?,
        Action::SetCashAssets(name, cash_assets) =>
            portfolio::set_cash_assets(&config, &name, cash_assets)?,

        Action::Show { name, flat } => portfolio::show(&config, &name, flat)?,
        Action::Rebalance { name, flat } => portfolio::rebalance(&config, &name, flat)?,

        Action::TaxStatement { name, year, tax_statement_path } =>
            tax_statement::generate_tax_statement(
                &config, &name, year, tax_statement_path.as_ref().map(String::as_str))?,
    };

    Ok(())
}