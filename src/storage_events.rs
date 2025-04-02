use iced::futures::channel::mpsc;
use iced::futures::Stream;
use iced::futures::StreamExt;
use sipper::{Never, Sipper, sipper};

/// Commands that can be sent to the storage
#[derive(Debug, Clone)]
pub enum StorageCommand {
    Updated,
}

/// Events that the storage can emit
#[derive(Debug, Clone)]
pub enum StorageEvent {
    Connected(mpsc::Sender<StorageCommand>),
    StorageUpdated,
}

/// Creates a sipper that listens for storage events
pub fn storage_sipper() -> impl Sipper<Never, StorageEvent> {
    sipper(async move |mut output| {
        println!("Storage sipper starting");
        // Create our channel
        let (sender, mut receiver) = mpsc::channel(100);

        // Send the sender back to the app
        output.send(StorageEvent::Connected(sender.clone())).await;
        println!("Sent Connected event");

        // Process commands and send events in an infinite loop
        loop {
            match receiver.next().await {
                Some(StorageCommand::Updated) => {
                    // Simulate processing time
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                    
                    // Send the StorageUpdated event
                    println!("Storage sipper: sending StorageUpdated event");
                    output.send(StorageEvent::StorageUpdated).await;
                }
                None => {
                    println!("Storage sipper: channel closed, waiting...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }
    })
}
