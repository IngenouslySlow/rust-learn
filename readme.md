# Basics of Rust

## Creating a basic Rust file -- hello.rs

Like in any programming language, you can create a sample file to execute the rust code inside or use a package manager. Rust also has a package manager which is included with it's installation called **Cargo**

Creating a rust file using `touch hello.rs`

```rust
// Basic function with rust
fn main(){
  println!("Beunos Dias Mi Abuelo!");
}
```

Now to run the file, use `rustc hello.rs`

## Creating a project using Cargo

To create a rust project use `cargo init`. This will generate your project scaffolding.

You can find src folder, a .gitignore file, and a cargo.toml file.

Inside the src folder you will find your **main.rs** file and this is your main file for your application

When you run `cargo run` this is the file that runs.

The very first time you run `cargo run` a new **target** folder will be generated which contains all the file outputs and everything else

This will be ignored in .gitignore file as you shouldn't upload this to git

You can also run `cargo build` to build your application for development and `cargo build --release` to release for production

## Setup

As discussed above **main.rs** is the main file of the application. You can create as many files you want in the src folder and include them in the main.rs file. When you run `cargo run` **main.rs** is the file it looks for.

### Including a file

Create another file inside src folder `touch print.rs`

create another function like so

```rust

pub fn run(){
  println!("Beunos Dias Mi Abuelo!");
}
```

Include it in **main.rs** file like so

```rust
mod print;
fn main() {
    print::run();
}
```

Every operation has a separate file created inside `src/`

Use `mod` keyword and then the file name without `.rs` extension

Access the functions the file has using **::** operator. For example `print::run()` in our case

Run `cargo run` to run the file
