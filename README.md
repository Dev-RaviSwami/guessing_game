# Rust Guessing Game

A simple, interactive command-line interface (CLI) guessing game built with Rust. 

The game generates a random number between 1 and 100, prompts the user for their name, and challenges them to guess the secret number. It provides hints along the way ("Too Big" or "Too Small") and tracks the number of attempts it takes to win.

## Features

- **Personalized Experience:** Asks for the player's name at the start.
- **Dynamic Hints:** Informs the player if their guess is too high or too low.
- **Attempt Tracking:** Keeps track of the total number of attempts.
- **Robust Input Handling:** Gracefully handles invalid inputs (non-numeric characters) without crashing, asking the player to type a valid number.

## Requirements

To run this project, you need to have **Rust** and **Cargo** installed. You can install them by following the instructions at [rustup.rs](https://rustup.rs/).

## Getting Started

1. Clone or navigate to the repository:
   ```bash
   cd guessing_game
   ```

2. Run the game using Cargo:
   ```bash
   cargo run
   ```

## How to Play

1. The game will first ask: `What is your name?`.
2. Enter your name and press Enter.
3. The game will greet you and prompt you to `Guess the number:`.
4. Enter a number between 1 and 100.
5. If your guess is incorrect, the game will tell you whether it was `Too Big` or `Too Small` and ask for another guess.
6. When you guess correctly, the game will display:
   ```text
   You Win!
   You guessed the number in X tries
   End of Game
   ```

## Dependencies

- [`rand`](https://crates.io/crates/rand) - Used to generate the random target number.
