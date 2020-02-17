use super::views::play_view::view::PlayView;
use std::time::Duration;
use tokio::time::interval;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::AsyncWriteExt;
use super::renderer::{Position};
use crate::renderer::Canvas;

pub struct Game {
    current_view: PlayView,
    running: bool,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            current_view: PlayView::default(),
            running: true,
        }
    }
}

impl Game {
    pub async fn render(state_handle: Arc<Mutex<Game>>) -> tokio::io::Result<()> {
        let mut screen = tokio::io::stdout();
        let mut render_loop = interval(Duration::from_millis(1000 / 60));
        let mut canvas = Canvas::default();
        screen.write_all(termion::clear::All.as_ref()).await?;
        screen.write_all(termion::cursor::Hide.as_ref()).await?;
        screen.flush().await?;
        loop {
            canvas.clear();
            render_loop.tick().await;
            {
                let game = state_handle.lock().await;
                if !game.running {
                    return Ok(());
                }

                game.current_view.render_at(&mut canvas, Position{x: 0, y: 0});
            }
            screen.write_all(canvas.to_printable_string().as_bytes()).await?;
            screen.flush().await?;
        }
    }

    pub async fn read_input(state_handle: Arc<Mutex<Game>>) {
        use crossterm::event::EventStream;
        use tokio::stream::StreamExt;

        let mut events = EventStream::new();

        loop {
            match events.next().await {
                Some(Ok(event)) => {
                    let mut game = state_handle.lock().await;
                    if !game.handle_input(&event) {
                        return;
                    }
                }
                _ => {
                    let mut game = state_handle.lock().await;
                    game.running = false;
                    return;
                }
            }
        }
    }

    fn handle_input(&mut self, event: &crossterm::event::Event) -> bool {
        use crossterm::event::{Event, KeyEvent, KeyCode};

        match event {
            Event::Key(KeyEvent { code, modifiers: _ }) => {
                match code {
                    KeyCode::Char('q') => {
                        self.running = false;
                        return false;
                    },
                    _ => self.current_view.handle_input(event)
                }
            }
            _ => ()
        };

        true
    }
}