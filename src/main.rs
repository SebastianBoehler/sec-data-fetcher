mod cli;
use clap::Parser;
use cli::{Arguments, Command, Input};
use sec_data_fetcher::{SecClient, extract_tables, parse_xml};
use serde::Serialize;
use std::{
    error::Error,
    io::{self, BufWriter, Write},
    process::ExitCode,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    match run(Arguments::parse()).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("sec-data-fetcher: {error}");
            ExitCode::FAILURE
        }
    }
}

async fn run(args: Arguments) -> Result<()> {
    match &args.command {
        Command::Lookup { ticker } => write_json(&client(&args)?.cik_lookup(ticker).await?),
        Command::Submissions { cik } => write_json(&client(&args)?.get_company_data(cik).await?),
        Command::Facts { cik } => write_json(&client(&args)?.get_company_facts(cik).await?),
        Command::Reports { cik, after, forms } => {
            let forms: Vec<_> = forms.iter().map(String::as_str).collect();
            write_json(&client(&args)?.get_reports(cik, *after, &forms).await?)
        }
        Command::Fetch { url } => {
            let content = client(&args)?.fetch_filing(url).await?;
            io::stdout().lock().write_all(content.as_bytes())?;
            Ok(())
        }
        Command::Tables(input) => write_json(&extract_tables(&read_input(input, &args).await?)),
        Command::Xml(input) => write_json(&parse_xml(&read_input(input, &args).await?)?),
    }
}

fn client(args: &Arguments) -> Result<SecClient> {
    let user_agent = args
        .user_agent
        .as_deref()
        .ok_or("Set SEC_USER_AGENT or --user-agent to your application name and contact email")?;
    Ok(SecClient::with_rate_limit(
        user_agent,
        args.requests_per_second,
    )?)
}

async fn read_input(input: &Input, args: &Arguments) -> Result<String> {
    match (&input.file, &input.url) {
        (Some(path), None) => Ok(std::fs::read_to_string(path)?),
        (None, Some(url)) => Ok(client(args)?.fetch_filing(url).await?),
        _ => Err("Choose exactly one of --file or --url".into()),
    }
}

fn write_json(value: &impl Serialize) -> Result<()> {
    let mut out = BufWriter::new(io::stdout().lock());
    serde_json::to_writer(&mut out, value)?;
    writeln!(out)?;
    out.flush()?;
    Ok(())
}
