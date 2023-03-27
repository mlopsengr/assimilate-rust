# assimilate-rust
Journey to learning Rust


## Installation
zsh - curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

zsh source "$HOME/.cargo/env"

- The above is used to install Rust on your machine, the below is used to install cargo, the Rust package manager.

zsh curl https://sh.rustup.rs -sSf | sh

zsh source "$HOME/.cargo/env"

- For the AWS Lambda example, install node.js [here](https://nodejs.org/en/download/)

- Then install AWS CLI [here](https://docs.aws.amazon.com/cli/latest/userguide/install-cliv2.html)

- Install the cdk library 
sh npm i aws-cdk-lib



## Rust Guide
- [Rust Book](https://doc.rust-lang.org/book/title-page.html)

- Rust is a statically typed language, meaning that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.