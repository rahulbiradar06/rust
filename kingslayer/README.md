# 👑 Kingslayer ⚔️
### Playing the game

```sh
cargo install kingslayer
kingslayer
```
or clone the project and run it:
```sh
cargo run --release
```

### Creating and running your own world

Worlds can be created with RON and Rust helper functions. Running a specific world on the command line looks like this:
```sh
kingslayer [FILE]
```
or with your own Rust code:
```rust
use kingslayer::Cli;

fn main() {
    let cli = Cli::from_file("worlds/world.ron");

    cli.start();
}
```
The game loop can also be managed manually like this:
```rust
use kingslayer::Cli;

fn main() {
    let cli = Cli::from_file("worlds/world.ron");

    println!("{}", cli.ask("l"));
    loop {
        let s = cli.ask(&Cli::prompt());
        println!("{}", s);
        if s.contains("Farewell.") {
            break;
        }
    }
}
```
This method allows for other forms of input and output such as within a website. The content for the world can also be passed as a raw string with `Cli::from_ron_str`.

### Dependencies
* Rust/Cargo ^1.59.0
