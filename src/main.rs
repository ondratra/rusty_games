mod tic_rust_toe;


fn main() {
    let mut cli = tic_rust_toe::cli_interface::CliInterface::new();
    cli.start_game();
}
