use shared::types::uom::UcreError;

fn execution(args: Vec<String>) -> Result<(), UcreError> {
    let cmd = match args.get(1) {
        Some(c) => c,
        None => return Err(UcreError::from_str("ucre: Failed to get a subcommand")),
    };
    match cmd.as_str() {
        "ucc" => {
            let filename = match args.get(2) {
                Some(name) => name,
                None => {
                    return Err(UcreError::from_str(
                        "ucre: Failed to get a filename, ucc requires one",
                    ))
                }
            };
            ucc::run(&filename)
        }
        c @ _ => Err(UcreError::new(format!("{c} is not a valid subcommand"))),
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if let Err(err) = execution(args) {
        println!("{}, exiting", err)
    }
}
