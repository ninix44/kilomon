use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, Paragraph},
    Frame,
};
use crate::app::{App, SortBy};

pub fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(f.size());

    let header_cells = [
        ("PID", SortBy::Pid),
        ("Name", SortBy::Name),
        ("CPU", SortBy::Cpu),
        ("Memory", SortBy::Memory)
    ]
        .iter()
        .map(|(title, sort_type)| {
            let color = if app.sort_by == *sort_type {
                Color::Green
            } else {
                Color::Yellow
            };

            Cell::from(*title).style(Style::default().fg(color).add_modifier(Modifier::BOLD))
        });

    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.processes.iter().map(|(pid, name, cpu, mem)| {
        let cpu_style = if *cpu > 50.0 {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else if *cpu > 10.0 {
            Style::default().fg(Color::LightYellow)
        } else {
            Style::default().fg(Color::Green)
        };

        let mem_mb = mem / 1024 / 1024;
        let mem_str = if mem_mb > 1024 {
            format!("{:.1} GB", mem_mb as f64 / 1024.0)
        } else {
            format!("{} MB", mem_mb)
        };

        Row::new(vec![
            Cell::from(pid.to_string()),
            Cell::from(name.clone()),
            Cell::from(format!("{:.1}%", cpu)).style(cpu_style),
            Cell::from(mem_str),
        ])
    });

    let t = Table::new(
        rows,
        [
            Constraint::Length(8),
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(30),
        ],
    )
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(" ⚡ KiloMon Process Manager "))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED).fg(Color::Cyan))
        .highlight_symbol(">> ");

    f.render_stateful_widget(t, rects[0], &mut app.state);

    let info_text = format!(
        "Total Processes: {} | [Up/Down]: Move | [K]: KILL | [S]: Sort | [Q]: Quit",
        app.processes.len()
    );

    let info = Paragraph::new(info_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::TOP));

    f.render_widget(info, rects[1]);
}