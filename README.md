# 🎲 Rust Guessing Game

This is a simple guessing game implemented in Rust. The game generates a random number using the `rand` crate and challenges the player to guess the correct number within a specified limit.

## 🌟 Features

- **Random Number Generation:** The game utilizes the `rand` crate to generate a random number.
  
- **Customizable Limit:** You can set the limit of the random number by providing it as the first command-line argument. By default, the limit is set to 100.

## 🎮 How to Play

1. **Clone the repository:**

   ```bash
   git clone https://github.com/adam-peter/guessing-game.git
   ```

2. **Navigate to the project directory:**

   ```bash
   cd guessing-game
   ```

3. **Build the project:**

   ```bash
   cargo build
   ```

4. **Run the game:**

   ```bash
   cargo run [limit]
   ```

   Replace `[limit]` with the desired limit for the random number (optional). If not provided, the default limit is 100.

5. **Follow the on-screen instructions to guess the correct number.**

## 🚀 Example

```bash
cargo run 50
```

This command runs the game with a random number limit of 50.

## 🤝 Contributing

Contributions are welcome! If you have any ideas for improvements or new features, feel free to open an issue or create a pull request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
