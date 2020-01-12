use super::views::play_view::view::PlayView;
use std::time::Duration;
use tokio::time::interval;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{AsyncWriteExt};

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
        let mut render_loop = interval(Duration::from_millis(1000 / 1));
        loop {
            render_loop.tick().await;
            screen.write_all(termion::clear::All.as_ref()).await?;
            {
                let game = state_handle.lock().await;
                if !game.running {
                    return Ok(());
                }
                game.current_view.render_at(&mut screen, 0, 0).await?;
            }
            &mut screen.flush().await?;
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
                        return
                    }
                },
                _ => {
                    let mut game = state_handle.lock().await;
                    game.running = false;
                    return;
                }
            }
        }
    }

    fn handle_input(&mut self, event: &crossterm::event::Event) -> bool {
//        println!("handle event {:#?}", event);
        use crossterm::event::{Event, KeyEvent, KeyCode};

        match event {
            Event::Key(KeyEvent{code, modifiers: _}) => {
                match code {
                    KeyCode::Char('q') => {
                        self.running = false;
                        false
                    }
                    _ => true
                }
            }
            _ => true
        }
    }
}