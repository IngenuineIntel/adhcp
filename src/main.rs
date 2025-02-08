// spyre
// main.rs

const VERSION: &'static str = "0.0.1";

use std::sync::mpsc::Receiver;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph},
};

mod thread;

static STYLE: u8 = 1;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    let log_receiver: Receiver<String>;
    let lease_receiver: Receiver<String>;
    (log_receiver, lease_receiver) = thread::start_collection_thread(); 

    let result = run(&mut terminal, log_receiver, lease_receiver);
    ratatui::restore();
    result
}

fn run(terminal: &mut ratatui::DefaultTerminal,
    log_receiver: Receiver<String>, 
    lease_receiver: Receiver<String> 
    ) -> std::io::Result<()> {

    

    loop {
        terminal.draw(|frame| draw(frame))?;
        if handle_events()? {
            break Ok(());
        }
    }
}

fn handle_events() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            // handle other key events
            _ => {}
        },
        // handle other events
        _ => {}
    }
    Ok(false)
}

fn style_default() -> (Style, Style, Style) {

    let border_style = Style::new()
        .blue()
        .add_modifier(Modifier::BOLD);
    let title_style = Style::new()
        .light_blue();
    let default_text_style = Style::new();

    (border_style, title_style, default_text_style)
}

fn style_matcha() -> (Style, Style, Style) {

    let border_style = Style::new()
        .green()
        .add_modifier(Modifier::BOLD);
    let title_style = Style::new()
        .green()
        .add_modifier(Modifier::BOLD);
    let default_text_style = Style::new()
        .add_modifier(Modifier::BOLD);
    
    (border_style, title_style, default_text_style)
}

fn draw(frame: &mut Frame) {
    use Constraint::{Fill, Length, Min};

    let border_style: Style;
    let title_style: Style;
    let default_text_style: Style;

    match STYLE {
        1 => {(border_style, title_style, default_text_style) = style_matcha();}
        _ => {(border_style, title_style, default_text_style) = style_default();}
    }

    let version_nr_size: u16 = (VERSION.chars().count() as u16) + 2;

    let vertical = Layout::vertical([Length(1), Min(0), Length(4)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let title_horizontal = Layout::horizontal([Min(0), Length(version_nr_size)]);
    let [left_area, right_area] = horizontal.areas(main_area);
    let [left_title_area, right_title_area] = title_horizontal.areas(title_area);

    frame.render_widget(Block::bordered().title("Spyre").style(border_style), left_title_area);
    frame.render_widget(Block::bordered().title(VERSION).style(border_style), right_title_area);
    frame.render_widget(Block::bordered().title("Log").style(border_style), status_area);
    frame.render_widget(Block::bordered().title("Left").style(border_style), left_area);
    frame.render_widget(Block::bordered().title("Right").style(border_style), right_area);
}
