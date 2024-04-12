use monitor_rs::app::{App, AppResult, SystemInfo};
use monitor_rs::event::{Event, EventHandler};
use monitor_rs::common::{format_bytes};
use monitor_rs::handler::handle_key_events;
use monitor_rs::tui::Tui;
use tokio::select;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use sysinfo::{
    Components, Disks, Networks, System,
};

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;


    let mut draw_interval = tokio::time::interval(std::time::Duration::from_millis(250));
    let mut info_interval = tokio::time::interval(std::time::Duration::from_millis(2000));
    loop {
        let draw_delay = draw_interval.tick();
        let info_delay = info_interval.tick();

        select! {
            _ = draw_delay => {
                tui.draw(&mut app)?;
                match tui.events.next().await? {
                    Event::Tick => app.tick(),
                    Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
                    Event::Mouse(_) => {}
                    Event::Resize(_, _) => {}
                }
                if !app.running {
                    break;
                }
            }
            _ = info_delay => {
                let sys = System::new_all();

                let mut system_info = SystemInfo::default();
                if let Some(host_name) = System::host_name() {
                    system_info.host_name = host_name;
                }
                system_info.total_memory = format_bytes(sys.total_memory());
                system_info.used_memory = format_bytes(sys.used_memory());
                system_info.cpus = format!("{}", sys.cpus().len());
                system_info.monitor_memory = Vec::from(app.system_info.monitor_memory);
                system_info.monitor_memory.push((sys.used_memory() as f64 / sys.total_memory() as f64 * 100.0) as u64);
                
                app.system_info = system_info;
            }
        }
    }
    // // Start the main loop.
    // while app.running {
    //     // Render the user interface.
    //     // Handle events.
        
    // }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
