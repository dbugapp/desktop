use iced::Alignment::Center;
use iced::futures::channel::mpsc;
use iced::widget::{button, center, column, text};
use iced::{Element, Subscription, Task};
use sipper::{Never, Sipper, sipper};

pub fn main() -> iced::Result {
    iced::application(
        "iced • counter sipper example with pause/resume",
        CounterApp::update,
        CounterApp::view,
    )
    .subscription(CounterApp::subscription)
    .window_size([400.0, 200.0])
    .run()
}

// Commands sent to the counter stream
#[derive(Debug, Clone)]
enum Command {
    TogglePause,
}

// A connection to the counter stream. Aliased for clarity, not necessary.
type Connection = mpsc::Sender<Command>;

// Events received from the counter stream
#[derive(Debug, Clone)]
enum Event {
    Connected(Connection),
    CounterUpdated(u32),
}

struct CounterApp {
    counter: u32,
    paused: bool,
    sender: Option<Connection>,
}

#[derive(Debug, Clone)]
enum Message {
    ChannelEvent(Event),
    TogglePause,
}

impl Default for CounterApp {
    fn default() -> Self {
        Self {
            counter: 0,
            paused: true,
            sender: None,
        }
    }
}

impl CounterApp {
    fn view(&self) -> Element<Message> {
        let button_text = match (self.paused, self.counter) {
            (true, 0) => "Play",
            (true, _) => "Resume",
            (false, _) => "Pause",
        };

        center(
            column![
                text(format!("{}", self.counter)).size(28).center(),
                button(text(button_text).center())
                    .on_press(Message::TogglePause)
                    .style(if self.paused {
                        button::success
                    } else {
                        button::danger
                    })
                    .width(150),
            ]
            .spacing(20)
            .align_x(Center)
            .width(150),
        )
        .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChannelEvent(event) => match event {
                Event::Connected(sender) => {
                    eprintln!("Connected to counter stream");
                    self.sender = Some(sender);
                }
                Event::CounterUpdated(count) => {
                    // Only update the counter if we're not paused to
                    // simulate an audio engine stopping immediately
                    // and discarding any pending updates from the engine
                    if !self.paused {
                        self.counter = count;
                    }
                }
            },
            Message::TogglePause => {
                if let Some(sender) = &mut self.sender {
                    let _ = sender.try_send(Command::TogglePause);
                    self.paused = !self.paused;
                }
            }
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(counter_sipper).map(Message::ChannelEvent)
    }
}

fn counter_sipper() -> impl Sipper<Never, Event> {
    sipper(async move |mut output| {
        // Create our channel
        let (sender, mut receiver) = mpsc::channel(100);

        // Send the sender back to the app
        let _ = output.send(Event::Connected(sender)).await;

        let mut counter = 0;
        let mut paused = true;

        // Process commands and send events
        loop {
            // Sleep for a bit to simulate work
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

            // Check for commands non-blocking
            match receiver.try_next() {
                Ok(Some(Command::TogglePause)) => {
                    eprintln!("toggling pause {}", paused);
                    paused = !paused;
                    // we could send a message back to the app here
                    // acknowledging the pause state change, but single source
                    // of truth is the app state
                    // let _ = output.send(Event::PlayPause(paused)).await;
                }
                _ => {} // No command or error, continue
            }

            // Update counter if not paused
            if !paused {
                counter += 1;
                let _ = output.send(Event::CounterUpdated(counter)).await;
            }
        }
    })
}

