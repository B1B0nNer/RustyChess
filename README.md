# RustyChess 🦀♟️

A terminal-based chess game written in Rust using the `ratatui` library.

RustyChess is a fully functional chess engine and UI that runs entirely in your terminal. It features a modern TUI (Terminal User Interface) with mouse support, move history, and captured pieces tracking.

## Features

- **Full Chess Rules**: 
    - All piece movements (Pawn, Knight, Bishop, Rook, Queen, King).
    - En Passant captures.
    - Check, Checkmate, and Stalemate detection.
    - Move validation (prevents moving into check).
- **Interactive TUI**:
    - Mouse support for selecting and moving pieces.
    - Visual move hints (dots indicating where a piece can move).
    - Dynamic turn indicator.
- **Side Panels**:
    - **History**: Keeps track of all moves in the current session.
    - **Captured Pieces**: Displays pieces taken by each player.
- **Game Management**:
    - **Warnings**: Visual alerts for Check and Checkmate.
    - **Replay**: Ability to restart the game once it's finished.

## Project Architecture

The project follows a modular design, separating the game logic from the terminal user interface.

### Core Components

1.  **Game State (`src/main.rs`)**:
    -   The `Game` struct is the central authority on the current state of the match.
    -   It manages the 8x8 board representation, the list of active pieces, move history, and turn-based logic.
    -   It handles high-level operations like selecting a piece, validating legal moves, and executing moves.

2.  **Piece Logic (`src/pieces/`)**:
    -   All chess pieces implement the `Piece` trait, which defines behavior for position tracking, move generation, and identification.
    -   Each piece type (Pawn, Rook, etc.) has its own implementation of move generation logic, including special moves like En Passant.

3.  **TUI Engine (`src/render_board/`)**:
    -   Built on top of `ratatui`, the UI is divided into several specialized components.
    -   `render_app.rs`: Manages the main application loop, terminal initialization, and event handling (keyboard and mouse).
    -   `render_board.rs`: Responsible for drawing the 8x8 grid, pieces (using custom ASCII art), and move hints.
    -   `history_panel.rs`, `captured_panel.rs`, `hint_panel.rs`: Specialized widgets for the sidebars and status messages.

### Architecture Diagram (High Level)

```text
       User Input (Mouse/Key)
               │
               ▼
    ┌───────────────────────┐
    │     Event Handler     │ (render_app.rs)
    └──────────┬────────────┘
               │
               ▼
    ┌───────────────────────┐      ┌──────────────────────┐
    │      Game Logic       │◄────►│   Piece Behaviors    │
    │      (Game Struct)    │      │ (Piece Trait Impls)  │
    └──────────┬────────────┘      └──────────────────────┘
               │
               ▼
    ┌───────────────────────┐
    │     TUI Renderer      │ (ratatui)
    └───────────────────────┘
```

### Technical Highlights

-   **Move Validation**: When a piece is selected, the `Game` struct queries the piece for its "potential" moves. It then filters these moves by simulating each one and checking if the resulting board state leaves the player's King in check.
-   **Check/Checkmate Detection**: The engine determines check by verifying if any opponent piece can attack the King's current square. Checkmate occurs if the King is in check and no legal moves exist to remove the threat.
-   **Terminal Interaction**: Leveraging `ratatui-interact`, the game provides a clickable board, making it feel more like a modern application than a traditional CLI tool.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- A terminal with support for true color and mouse events (e.g., Alacritty, iTerm2, Kitty, or modern Windows Terminal).

### Build and Run

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/RustyChess.git
   cd RustyChess
   ```

2. Run the game:
   ```bash
   cargo run
   ```

## How to Play

1. **Selecting a Piece**: Click on a piece of your color. Valid moves will be highlighted with a `hint` indicator on empty squares.
2. **Moving a Piece**: Click on one of the highlighted squares or an opponent's piece to capture it.
3. **Quitting**: Press `q` at any time to exit the game.
4. **Restarting**: When the game ends (Checkmate or Stalemate), a **REPLAY** button will appear at the bottom. Click it to start a new match.

## Releases

The latest binary for various platforms can be found in the [Releases](https://github.com/your-username/RustyChess/releases) section. Alternatively, you can build a release binary locally:

```bash
cargo build --release
```
The optimized executable will be located at `target/release/RustyChess`.

## License

This project is open-source and available under the MIT License.
