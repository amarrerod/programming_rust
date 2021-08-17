mod args;
mod file_handlers;
mod replacer;

fn main() {
    let args = args::parse_args();
    let data: String = file_handlers::read_file(&args.filename);
    let replaced_data = replacer::replace(&args.target, &args.replacement, &data);
    file_handlers::write_to_file(&args.output, &replaced_data);
}
