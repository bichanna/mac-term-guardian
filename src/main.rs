use crossterm::{
    event,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use localauthentication_rs::{LAPolicy, LocalAuthentication};
// use nokhwa::{
//     pixel_format::RgbFormat,
//     utils::{CameraIndex, RequestedFormat, RequestedFormatType},
//     Camera,
// };
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    style::{Style, Stylize},
    widgets::Clear,
};
use std::env;
use std::io::stdout;
use tui_big_text::{BigTextBuilder, PixelSize};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Handle all these Results.

    // Get username.
    let username = env::var("USER").unwrap_or("there".to_string());
    let mut message = BigTextBuilder::default()
        .pixel_size(PixelSize::Full)
        .style(Style::new().blue())
        .lines(vec!["Hello".red().into(), username.red().into()])
        .build()?;

    // Create an alternate screen
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    // Create a new terminal screen instance.
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // Create a new instance of authentication thingy.
    let authentication = LocalAuthentication::new();

    // Main loop.
    loop {
        // Drawing.
        terminal.draw(|frame| {
            frame.render_widget(message.clone(), frame.size());
        })?;

        // Handling events.
        if event::poll(std::time::Duration::from_millis(50))? {
            let good = authentication
                .evaluate_policy(LAPolicy::DeviceOwnerAuthentication, "authenticate you");

            if good {
                break;
            } else {
                // TODO: Shoot a video and then warn the actual user.
                // let cam_idx = CameraIndex::Index(0);
                // let requested = RequestedFormat::new::<RgbFormat>(
                //     RequestedFormatType::AbsoluteHighestFrameRate,
                // );
                // let mut camera = Camera::new(cam_idx, requested).unwrap();
                // let frame = camera.frame()?;
                // let decoded = frame.decode_image::<RgbFormat>()?;

                terminal.draw(|frame| {
                    let size = frame.size();
                    // Clear the terminal.
                    frame.render_widget(Clear, size);
                    // Draw 'GET OUT' text.
                    message = BigTextBuilder::default()
                        .pixel_size(PixelSize::Full)
                        .style(Style::new().red())
                        .lines(vec!["GET".black().into(), "OUT".black().into()])
                        .build()
                        .unwrap();
                })?;
            }
        }
    }

    // Destroy the alternate screen.
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
