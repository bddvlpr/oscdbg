use std::{io, time::Duration};

use chrono::Local;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, ListItem, Widget},
    Frame,
};

use crate::{
    cli::Args,
    listener::{create_listener, Listener, ListenerMessage},
    log::{LogEntry, StatefulLog},
    tui,
};

pub struct App {
    args: Args,
    listener: Listener,
    log: StatefulLog,
    running_state: RunningState,
}

#[derive(PartialEq)]
enum RunningState {
    Running,
    Exiting,
}

impl App {
    pub fn new(args: Args) -> Self {
        let addr = format!("{}:{}", args.address, args.port)
            .parse()
            .expect("Unable to parse the socket address");
        let listener = create_listener(addr);

        Self {
            args,
            listener,
            log: StatefulLog::default(),
            running_state: RunningState::Running,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while self.running_state != RunningState::Exiting {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
            self.handle_channel_events();
        }
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if !event::poll(Duration::from_millis(100))? {
            return Ok(());
        }
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            KeyCode::Char('h') | KeyCode::Left => self.log.unselect(),
            KeyCode::Char('j') | KeyCode::Down => self.log.next(),
            KeyCode::Char('k') | KeyCode::Up => self.log.previous(),
            KeyCode::Char('g') => self.go_top(),
            KeyCode::Char('G') => self.go_bottom(),
            _ => {}
        }
    }

    fn handle_channel_events(&mut self) {
        if let Ok(message) = self.listener.try_recv() {
            match message {
                ListenerMessage::PacketReceived(packet, addr) => {
                    self.log.items.push(LogEntry(Local::now(), packet, addr));
                }
                ListenerMessage::Error(err) => {
                    // TODO: naurr
                    eprintln!("{err}");
                    panic!();
                }
            }
        }
    }

    fn go_top(&mut self) {
        self.log.state.select(Some(0));
    }

    fn go_bottom(&mut self) {
        self.log.state.select(Some(self.log.items.len() - 1));
    }

    fn exit(&mut self) {
        self.running_state = RunningState::Exiting;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let selected = self.log.state.selected().is_some();
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(if selected { 30 } else { 100 }),
                Constraint::Fill(1),
            ])
            .split(area);

        let block = Block::new().borders(Borders::ALL).title("Log");

        let items: Vec<ListItem> = self
            .log
            .items
            .iter()
            .enumerate()
            .map(|(i, entry)| entry.to_list_item(i))
            .collect();

        List::new(items)
            .highlight_symbol(">")
            .render(block.inner(area), buf);
        block.render(layout[0], buf);

        if selected {
            Block::new()
                .borders(Borders::ALL)
                .title("Inspector")
                .render(layout[1], buf);
        }
    }
}
