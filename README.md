# Clap Main

This crate provides an procmacro `#[clap_main]` to decorate your entry point function and automatically 
parse a struct that derives clap::Parser from the cli args. 

## Example Usage
```rust

#[derive(clap::Parse)]
struct CliArgs {
  /// A name to be greeted
  name: String
}

#[clap_main]
pub fn run(args: CliArgs) -> Result<(), Box<dyn std::error::Error>> {
  println!("Hello {name}");  

  Ok(())
}
```

