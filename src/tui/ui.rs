use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap,
    },
    Frame,
};

use super::app::{App, AppMode, InputMode};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Main content
            Constraint::Length(3),  // Status bar
        ].as_ref())
        .split(f.area());

    draw_header(f, chunks[0], app);
    draw_main_content(f, chunks[1], app);
    draw_status_bar(f, chunks[2], app);

    if app.show_help {
        draw_help_popup(f, app);
    }
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    let title = match app.mode {
        AppMode::Normal => "Claw - Task Manager",
        AppMode::Insert => "Claw - Add New Task",
        AppMode::Help => "Claw - Help",
    };

    let header = Paragraph::new(title)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        );

    f.render_widget(header, area);
}

fn draw_main_content(f: &mut Frame, area: Rect, app: &App) {
    match app.input_mode {
        InputMode::Normal => draw_task_list(f, area, app),
        InputMode::Editing => draw_input_form(f, area, app),
    }
}

fn draw_task_list(f: &mut Frame, area: Rect, app: &App) {
    let tasks: Vec<ListItem> = app
        .task_list
        .tasks
        .iter()
        .enumerate()
        .map(|(_i, task)| {
            let status_icon = if task.completed { "✓" } else { "○" };
            let style = if task.completed {
                Style::default().fg(Color::Green).add_modifier(Modifier::CROSSED_OUT)
            } else {
                Style::default().fg(Color::White)
            };

            let content = format!("{} [{}] {}", status_icon, task.id, task.description);
            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let title = format!("Tasks ({}/{})", 
        app.task_list.tasks.len(), 
        app.task_list.tasks.iter().filter(|t| !t.completed).count()
    );

    let tasks_list = List::new(tasks)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .style(Style::default().fg(Color::White)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");

    let mut state = ListState::default();
    state.select(Some(app.selected_task));

    f.render_stateful_widget(tasks_list, area, &mut state);

    // Show empty state message
    if app.task_list.tasks.is_empty() {
        let empty_msg = Paragraph::new("No tasks yet!\n\nPress 'a' to add your first task")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        let inner_area = area.inner(Margin {
            vertical: area.height / 3,
            horizontal: area.width / 4,
        });

        f.render_widget(empty_msg, inner_area);
    }
}

fn draw_input_form(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Input field
            Constraint::Min(0),     // Spacer
        ].as_ref())
        .margin(2)
        .split(area);

    let input = Paragraph::new(app.input_buffer.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("New Task Description")
                .style(Style::default().fg(Color::White)),
        );

    f.render_widget(input, chunks[0]);

    // Set cursor position
    f.set_cursor_position((
        chunks[0].x + app.input_buffer.len() as u16 + 1,
        chunks[0].y + 1,
    ));

    // Instructions
    let instructions = Paragraph::new("Enter: Save task | Esc: Cancel")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    f.render_widget(instructions, chunks[1]);
}

fn draw_status_bar(f: &mut Frame, area: Rect, app: &App) {
    let status_text = match app.input_mode {
        InputMode::Normal => {
            let shortcuts = "a:add | Space:toggle | d:delete | h:help | q:quit";
            format!("{} | {}", app.status_message, shortcuts)
        }
        InputMode::Editing => app.status_message.clone(),
    };

    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Gray)),
        );

    f.render_widget(status, area);
}

fn draw_help_popup(f: &mut Frame, app: &App) {
    let help_text = app.get_help_text();
    
    let help_lines: Vec<Line> = help_text
        .iter()
        .map(|&line| {
            if line.is_empty() {
                Line::from("")
            } else if line.ends_with(':') {
                Line::from(Span::styled(line, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
            } else if line.starts_with("  ") {
                let parts: Vec<&str> = line.splitn(2, " - ").collect();
                if parts.len() == 2 {
                    Line::from(vec![
                        Span::styled(parts[0], Style::default().fg(Color::Yellow)),
                        Span::styled(" - ", Style::default().fg(Color::White)),
                        Span::styled(parts[1], Style::default().fg(Color::White)),
                    ])
                } else {
                    Line::from(Span::styled(line, Style::default().fg(Color::White)))
                }
            } else {
                Line::from(Span::styled(line, Style::default().fg(Color::White)))
            }
        })
        .collect();

    let help_text = Text::from(help_lines);

    let popup_area = centered_rect(60, 70, f.area());

    f.render_widget(Clear, popup_area);
    
    let help_paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .title("Help")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(help_paragraph, popup_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}