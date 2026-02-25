# Rustacean Chronicles

An interactive, story-driven tutorial sandbox built with Rust and WebAssembly, designed to teach Rust concepts directly in the browser! 🦀✨

## Overview

Rustacean Chronicles is a lightweight, purely client-side WebAssembly application. It features 9 episodes of fully guided curriculum, complete with:
- A beautiful, glassmorphism-inspired dark mode UI.
- Direct inline code editing via interactive modules.
- A built-in terminal validation engine that checks your answers on the fly.

## Prerequisites

To run this project locally, you'll need the following installed:

1. **Rust & Cargo**
   Install Rust through `rustup`:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   
2. **WebAssembly Target**
   Add the WebAssembly target to your Rust toolchain:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **Trunk**
   Trunk is the build tool used for this WebAssembly application.
   ```bash
   cargo install --locked trunk
   ```

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/anindyatheone/rustacean-chronicles.git
   cd rustacean-chronicles
   ```

2. **Run the development server:**
   ```bash
   trunk serve
   ```
   
   Trunk will compile the Rust project into WebAssembly and serve it on a local development server (usually `http://localhost:8080` unless specified otherwise in your terminal output).

3. **Explore the tutorial!**
   Open your browser to the local address provided by Trunk and begin your journey as a Starship Engineer!

## Project Structure

- `src/main.rs`: The application entry point, routing logic, and state machine.
- `src/components/`:
  - `story.rs`: Renders the narrative text and episode titles.
  - `workspace.rs`: The core interactive editor and terminal pane component.
- `src/validation/engine.rs`: The pure-Rust validation engine that cross-references user inputs with correct expected strings for each interactive module module.
- `style.css`: The central stylesheet for the premium dark-mode theme.

## Architecture Notes
This project utilizes [Leptos](https://github.com/leptos-rs/leptos) for its reactive UI and purely client-side evaluation architecture. It does not send code to a backend server to compile; instead, it checks predefined validation patterns against the user's inputs directly in the browser.

## License
MIT
