use clap::Parser;
use std::fs;

mod openapi;
mod slice;
use openapi::decode_spec;
use slice::{get_path, write_slice_to_file};

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let openapi_spec = fs::read_to_string(args.filepath)?;
    let decoded_spec = decode_spec(&openapi_spec);
    let path_item = get_path(&decoded_spec, &args.path_item_name);
    write_slice_to_file(&path_item, &args.output)
}

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    path_item_name: String,
    filepath: String,
    output: String,
}
