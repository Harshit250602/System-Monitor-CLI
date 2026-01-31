use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap, Tabs, Table, Row, Cell, Chart, Dataset, Axis},
    symbols,
    Frame,
};
use crate::app::{App, Tab};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3), // Tabs
                Constraint::Min(0),    // Content
            ]
            .as_ref(),
        )
        .split(f.area());

    draw_tabs(f, app, chunks[0]);

    match app.current_tab {
        Tab::Overview => draw_overview(f, app, chunks[1]),
        Tab::Processes => draw_processes(f, app, chunks[1]),
        Tab::Network => draw_network(f, app, chunks[1]),
    }
}

fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = ["Overview", "Processes", "Network"]
        .iter()
        .cloned()
        .map(Line::from)
        .collect();
    
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.current_tab as usize)
        .highlight_style(Style::default().fg(Color::Yellow));
    f.render_widget(tabs, area);
}

fn draw_overview(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Header
                Constraint::Percentage(50), // CPU & Memory
                Constraint::Percentage(50), // Disks
            ]
            .as_ref(),
        )
        .split(area);

    draw_header(f, chunks[0]);
    draw_cpu_mem(f, app, chunks[1]);
    draw_disks(f, app, chunks[2]);
}

fn draw_processes(f: &mut Frame, app: &App, area: Rect) {
    let header_cells = ["PID", "Name", "CPU", "Mem"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::DarkGray))
        .height(1)
        .bottom_margin(1);
    
    let rows = app.processes.iter().take(20).map(|item| {
        let cells = vec![
            Cell::from(item.pid.to_string()),
            Cell::from(item.name.clone()),
            Cell::from(format!("{:.1}%", item.cpu)),
            Cell::from(format!("{} MB", item.memory / 1024 / 1024)),
        ];
        Row::new(cells).height(1)
    });
    
    let t = Table::new(rows, [
        Constraint::Length(8),
        Constraint::Min(10),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title("Processes"));
    
    f.render_widget(t, area);
}

fn draw_network(f: &mut Frame, app: &App, area: Rect) {
    let rx_data: Vec<(f64, f64)> = app.network_history.iter().enumerate().map(|(i, (rx, _))| (i as f64, *rx)).collect();
    let tx_data: Vec<(f64, f64)> = app.network_history.iter().enumerate().map(|(i, (_, tx))| (i as f64, *tx)).collect();
    
    let max_val = app.network_history.iter().map(|(rx, tx)| rx.max(*tx)).fold(0.0, f64::max).max(100.0);

    let datasets = vec![
        Dataset::default()
            .name("RX")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Cyan))
            .data(&rx_data),
        Dataset::default()
            .name("TX")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Magenta))
            .data(&tx_data),
    ];

    let chart = Chart::new(datasets)
        .block(Block::default().title("Network Traffic (Bytes)").borders(Borders::ALL))
        .x_axis(Axis::default()
            .title("Time")
            .style(Style::default().fg(Color::Gray))
            .bounds([0.0, 100.0])
            .labels(vec![Span::raw("0"), Span::raw("100")]))
        .y_axis(Axis::default()
            .title("Bytes")
            .style(Style::default().fg(Color::Gray))
            .bounds([0.0, max_val])
            .labels(vec![Span::raw("0"), Span::from(format!("{:.0}", max_val))]));
    
    f.render_widget(chart, area);
}

fn draw_header(f: &mut Frame, area: Rect) {
    let text = Line::from(vec![
        Span::styled("System Monitor", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" | Press "),
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD).fg(Color::Blue)),
        Span::raw(" to exit | "),
        Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to switch tabs"),
    ]);
    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Info"))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(paragraph, area);
}

fn draw_cpu_mem(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    // CPU Usage
    let cpu_usage = app.system.global_cpu_usage();
    let cpu_gauge = Gauge::default()
        .block(Block::default().title("CPU Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(cpu_usage as u16);
    f.render_widget(cpu_gauge, chunks[0]);

    // Memory Usage
    let used_mem = app.system.used_memory();
    let total_mem = app.system.total_memory();
    let mem_percent = (used_mem as f64 / total_mem as f64 * 100.0) as u16;
    let mem_gauge = Gauge::default()
        .block(Block::default().title("Memory Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Blue))
        .percent(mem_percent)
        .label(format!("{}/{} MB", used_mem / 1024 / 1024, total_mem / 1024 / 1024));
    f.render_widget(mem_gauge, chunks[1]);
}

fn draw_disks(f: &mut Frame, app: &App, area: Rect) {
    let disks = app.disks.list();
    let mut disk_info = String::new();
    for disk in disks {
        disk_info.push_str(&format!(
            "{}: Total: {} GB, Available: {} GB\n",
            disk.name().to_string_lossy(),
            disk.total_space() / 1024 / 1024 / 1024,
            disk.available_space() / 1024 / 1024 / 1024
        ));
    }

    let paragraph = Paragraph::new(disk_info)
        .block(Block::default().title("Disks").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
