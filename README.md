# square-root-irrational
A rust crate to check if the square root of an integer is rational or not

## Install
```sh
cargo add --git https://github.com/adridevelopsthings/square-root-irrational 
```

## How to run

```rust
use square_root_irrational::is_square_root_rational;

let is_rational = is_square_root_rational(16);

match is_rational {
    true => println!("The square root of this number is rational"),
    false => println!("The square root of this number is irrational")
};
```