use chess_template::{Colour, Game, PieceType, Position};
/**
 * Chess GUI .
 * Author: Vilhelm Prytz <vilhelm@prytznet.se> / <vprytz@kth.se>
 */
use ggez::{conf, event, graphics, Context, ContextBuilder, GameError, GameResult};
use std::{collections::HashMap, path};

/// A chess board is 8x8 tiles.
const GRID_SIZE: i16 = 8;
/// Sutible size of each tile.
const GRID_CELL_SIZE: (i16, i16) = (90, 90);

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE as f32 * GRID_CELL_SIZE.1 as f32,
);

// GUI Color representations
const BLACK: graphics::Color =
    graphics::Color::new(228.0 / 255.0, 196.0 / 255.0, 108.0 / 255.0, 1.0);
const WHITE: graphics::Color =
    graphics::Color::new(188.0 / 255.0, 140.0 / 255.0, 76.0 / 255.0, 1.0);

/// GUI logic and event implementation structure.
///
struct AppState {
    sprites: HashMap<(Colour, PieceType), graphics::Image>, // For easy access to the apropriate PNGs
    game: Game, // Save piece positions, which tiles has been clicked, current colour, etc...
    positions: Vec<Position>, // Save the position of each tile
    selected_position: Option<Position>, // hold position of the selected piece
}

impl AppState {
    /// Initialise new application, i.e. initialise new game and load resources.
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        // A cool way to instantiate the board
        // You can safely delete this if the chess-library already does this

        let state = AppState {
            sprites: AppState::load_sprites(ctx),
            game: Game::new(),
            positions: Vec::new(),
            selected_position: None,
        };

        Ok(state)
    }
    #[rustfmt::skip] // Skips formatting on this function (not recommended)
                     /// Loads chess piese images into hashmap, for ease of use.
    fn load_sprites(ctx: &mut Context) -> HashMap<(Colour, PieceType), graphics::Image> {

        [
            ((Colour::Black, PieceType::King), "/black_king.png".to_string()),
            ((Colour::Black, PieceType::Queen), "/black_queen.png".to_string()),
            ((Colour::Black, PieceType::Rook), "/black_rook.png".to_string()),
            ((Colour::Black, PieceType::Pawn), "/black_pawn.png".to_string()),
            ((Colour::Black, PieceType::Bishop), "/black_bishop.png".to_string()),
            ((Colour::Black, PieceType::Knight), "/black_knight.png".to_string()),
            ((Colour::White, PieceType::King), "/white_king.png".to_string()),
            ((Colour::White, PieceType::Queen), "/white_queen.png".to_string()),
            ((Colour::White, PieceType::Rook), "/white_rook.png".to_string()),
            ((Colour::White, PieceType::Pawn), "/white_pawn.png".to_string()),
            ((Colour::White, PieceType::Bishop), "/white_bishop.png".to_string()),
            ((Colour::White, PieceType::Knight), "/white_knight.png".to_string())
        ]
            .iter()
            .map(|(piece, path)| {
                (*piece, graphics::Image::new(ctx, path).unwrap())
            })
            .collect::<HashMap<(Colour, PieceType), graphics::Image>>()
    }
}

// This is where we implement the functions that ggez requires to function
impl event::EventHandler<GameError> for AppState {
    /// For updating game logic, which front-end doesn't handle.
    /// It won't be necessary to touch this unless you are implementing something that's not triggered by the user, like a clock
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Draw interface, i.e. draw game board
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clear interface with gray background colour
        graphics::clear(ctx, [0.5, 0.5, 0.5, 1.0].into());

        let splash_text: String;

        // if game state is GameOver, draw game over screen
        if self.game.get_game_state() == chess_template::GameState::GameOver {
            splash_text = "Game Over, press R to restart!".to_string();
        } else {
            splash_text = format!(
                "Game is {:?}, it's {:?} turn.",
                self.game.get_game_state(),
                self.game.get_active_colour()
            );
        }

        // create text representation
        let state_text = graphics::Text::new(
            graphics::TextFragment::from(splash_text).scale(graphics::PxScale { x: 30.0, y: 30.0 }),
        );

        // get size of text
        let text_dimensions = state_text.dimensions(ctx);
        // create background rectangle with white coulouring
        let background_box = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                (SCREEN_SIZE.0 - text_dimensions.w as f32) / 2f32 as f32 - 8.0,
                (SCREEN_SIZE.0 - text_dimensions.h as f32) / 2f32 as f32,
                text_dimensions.w as f32 + 16.0,
                text_dimensions.h as f32,
            ),
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;

        // draw background
        graphics::draw(ctx, &background_box, graphics::DrawParam::default())
            .expect("Failed to draw background.");

        // draw grid
        for row in 0..8 {
            for col in 0..8 {
                // draw tile
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(
                        col * GRID_CELL_SIZE.0 as i32,
                        row * GRID_CELL_SIZE.1 as i32,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ),
                    match col % 2 {
                        0 => {
                            if row % 2 == 0 {
                                WHITE
                            } else {
                                BLACK
                            }
                        }
                        _ => {
                            if row % 2 == 0 {
                                BLACK
                            } else {
                                WHITE
                            }
                        }
                    },
                )
                .expect("Failed to create tile.");
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
                    .expect("Failed to draw tiles.");

                // convert row and col to idx
                let idx = row * 8 + col;

                if let Some(piece) = self.game.get_board()[idx as usize] {
                    graphics::draw(
                        ctx,
                        self.sprites.get(&(piece.colour, piece.piece_type)).unwrap(),
                        graphics::DrawParam::default()
                            .scale([2.0, 2.0]) // Tile size is 90 pixels, while image sizes are 45 pixels.
                            .dest([
                                col as f32 * GRID_CELL_SIZE.0 as f32,
                                row as f32 * GRID_CELL_SIZE.1 as f32,
                            ]),
                    )
                    .expect("Failed to draw piece.");
                }

                // draw dot on possible moves for selected piece
                if self
                    .positions
                    .contains(&Position::new(row as usize, col as usize).unwrap())
                {
                    let dot = graphics::Mesh::new_circle(
                        ctx,
                        graphics::DrawMode::fill(),
                        [
                            col as f32 * GRID_CELL_SIZE.0 as f32 + 45.0,
                            row as f32 * GRID_CELL_SIZE.1 as f32 + 45.0,
                        ],
                        10.0,
                        0.1,
                        [1.0, 0.0, 0.0, 1.0].into(),
                    )
                    .expect("Failed to create dot.");
                    graphics::draw(ctx, &dot, graphics::DrawParam::default())
                        .expect("Failed to draw dot.");
                }
            }
        }

        // draw text with dark gray colouring and center position
        graphics::draw(
            ctx,
            &state_text,
            graphics::DrawParam::default()
                .color([0.0, 0.0, 0.0, 1.0].into())
                .dest(ggez::mint::Point2 {
                    x: (SCREEN_SIZE.0 - text_dimensions.w as f32) / 2f32 as f32,
                    y: (SCREEN_SIZE.0 - text_dimensions.h as f32) / 2f32 as f32,
                }),
        )
        .expect("Failed to draw text.");

        // render updated graphics
        graphics::present(ctx).expect("Failed to update graphics.");

        Ok(())
    }

    /// Update game on mouse click
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) {
        if button == event::MouseButton::Left {
            /* check click position and update board accordingly */
            // each tile is 90x90 pixels, so we can divide the click position by 90 to get the tile
            let row = (y / GRID_CELL_SIZE.1 as f32) as usize;
            let col = (x / GRID_CELL_SIZE.0 as f32) as usize;

            // convert row, col to idx
            let idx = row * 8 + col;

            // check if the selected position has a piece and that it's the player's turn
            if let Some(piece) = self.game.get_board()[idx] {
                if piece.colour == self.game.get_active_colour() {
                    // convert row and column to Position
                    let position = Position::new(row, col);

                    // get possible moves for the selected piece
                    let available_moves = self.game.get_possible_moves(position.unwrap(), 0);

                    // set available moves to App State
                    self.positions = available_moves;

                    // set selected position to App State
                    self.selected_position = Some(Position::new(row, col).unwrap());
                }
            }

            // check if clicked position is in self.positions
            if self.positions.contains(&Position::new(row, col).unwrap()) {
                // print something cool!

                let new_game_state = self.game.make_move_pos(
                    self.selected_position.unwrap(),
                    Position::new(row, col).unwrap(),
                );

                // if new_game_state.is_ok(), then the move was successful and we remove the selected position
                if new_game_state.is_ok() {
                    self.selected_position = None;
                    self.positions = vec![];
                }
            }
        }
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        key: event::KeyCode,
        _mods: event::KeyMods,
        _: bool,
    ) {
        match key {
            // Quit if escape is pressed
            event::KeyCode::Escape => {
                event::quit(ctx);
            }
            event::KeyCode::R => {
                self.game = Game::new();
                self.positions = vec![];
                self.selected_position = None;
            }
            _ => (),
        }
    }
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");

    let context_builder = ContextBuilder::new(
        "schack",
        "Vilhelm Prytz <vilhelm@prytznet.se> / <vprytz@kth.se>",
    )
    .add_resource_path(resource_dir) // Import image files to GGEZ
    .window_setup(
        conf::WindowSetup::default()
            .title("Schack") // Set window title "Schack"
            .icon("/icon.png"), // Set application icon
    )
    .window_mode(
        conf::WindowMode::default()
            .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimensions
            .resizable(false), // Fixate window size
    );
    let (mut contex, event_loop) = context_builder.build().expect("Failed to build context.");

    let state = AppState::new(&mut contex).expect("Failed to create state.");
    event::run(contex, event_loop, state) // Run window event loop
}
