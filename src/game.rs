use std::time::{Duration};
use tokio::time::interval;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::AsyncWriteExt;
use crate::rendering::renderer::{Canvas, Position};
use crate::views::views::PlayView;

pub struct Game {
    pub current_view: PlayView,
    pub running: bool,
}

impl Game {
    pub fn create() -> Arc<Mutex<Game>> {
        Arc::new(Mutex::new(Game {
            current_view: PlayView::default(),
            running: true,
        }))
    }

    pub async fn run(game_handle: Arc<Mutex<Game>>) {
        let read_input = Self::read_input(game_handle.clone());
        let render = Self::render(game_handle.clone());
        tokio::spawn(render);
        {
            let mut game = game_handle.lock().await;
            game.current_view.on_create(game_handle.clone());
        }
        read_input.await;
    }

    pub async fn render(state_handle: Arc<Mutex<Game>>) -> tokio::io::Result<()> {
        let mut screen = tokio::io::stdout();
        let mut render_loop = interval(Duration::from_millis(1000 / 30));
        let mut canvas = Canvas::default();

        loop {
            canvas.clear();
            render_loop.tick().await;
            {
                let game = state_handle.lock().await;
                if !game.running {
                    return Ok(());
                }
                game.current_view.render_at(&mut canvas, Position { x: 0, y: 0 });
            }
            screen.write_all(canvas.get_printable_string().as_bytes()).await?;
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
                    }
                    _ => self.current_view.handle_input(event)
                }
            }
            _ => ()
        };

        true
    }
}