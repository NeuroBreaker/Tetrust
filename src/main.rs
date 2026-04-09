mod tetris;

fn main() {
    let result = tetris::run().unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
}
