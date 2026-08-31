use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

use crate::{app::{App, Section}, 
    resume::{Certification, Competition, Project, Activity, Experience}};

pub fn centered_rect(area: Rect, max_width: u16, max_height: u16) -> Rect {
    let width = area.width.min(max_width);
    let height = area.height.min(max_height);
    let x = (area.width - width) / 2;
    let y = (area.height - height) / 2;
    Rect::new(x, y, width, height)
}

const BODY_STYLE: Style = Style::new().fg(Color::White).bg(Color::Black);

/// Render a single card (bordered block with a title and scrollable text)
fn render_card(frame: &mut Frame, text: String, title: &str, offset: usize, area: Rect) {
    let block = styled_block(title);

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(BODY_STYLE)
        .scroll((offset as u16, 0))
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

/// Render the style of the my 'default' block
fn styled_block(title: impl Into<String> ) -> Block<'static> {
    Block::bordered()
        .title(title.into())
        .border_style(Style::default().fg(Color::Gray))
        .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
}

/// Render the footer with navigation hints
fn render_footer(frame: &mut Frame, area: Rect) {
    let text = "[← →] Sections  [↑ ↓] Scroll  [q/Esc] Quit";
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::Gray).bg(Color::DarkGray))
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}

/// Render the Vec<section> fields
fn render_paginated<T>(
    frame: &mut Frame,
    area: Rect,
    section: &str,
    items: &[T],
    scroll_offset: usize,
    card_height: usize,
    empty_msg: &str,
    scroll_msg: &str,
    format_item: impl Fn(&T) -> (String, String), // (title, body)
) {
    if items.is_empty() {
        let block = styled_block(section);
        let placeholder = Paragraph::new(empty_msg)
                .block(block)
                .style(BODY_STYLE);
            frame.render_widget(placeholder, area);
            return;
    }
    let available_height = area.height as usize;
    let max_visible = available_height / card_height;
    let total = items.len();
    
    let max_scroll = total.saturating_sub(max_visible);
    let offset = scroll_offset.min(max_scroll);
    
    let start = offset;
    let end = (start + max_visible).min(total);

    let mut constraints = Vec::new();
    for _ in start..end {
        constraints.push(Constraint::Length(card_height as u16));
    }
    
    if constraints.is_empty() {
        let block = styled_block(section);
        let placeholder = Paragraph::new(scroll_msg)
            .block(block)
            .style(BODY_STYLE);
        frame.render_widget(placeholder, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);
    
    for (i, chunk) in chunks.iter().enumerate() {
        let (title,body) = format_item(&items[start + i]);
    
        let block = styled_block(title);
        let paragraph = Paragraph::new(body)
            .block(block)
            .style(BODY_STYLE)
            .wrap(Wrap { trim: true });
        frame.render_widget(paragraph, *chunk);
    }
}

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    // Split area into: content (top) and footer (bottom 1 line)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
        .split(area);
    let content_area = chunks[0];
    let footer_area = chunks[1];

    // Render the footer (always the same)
    render_footer(frame, footer_area);

    // Render the current section inside the content area
    let section = &app.sections[app.curr_section];
    match section {
        Section::Personal => {
            let mut contact_text = format!("Email: {}\n", app.data.personal.email);
            if let Some(phone) = &app.data.personal.phone_number {
                contact_text.push_str(&format!("Phone: {}\n", phone));  
            }
            if let Some(location) = &app.data.personal.location {
                contact_text.push_str(&format!("Location: {}\n", location));
            }
            contact_text.push_str(&format!("GitHub: {}\n", app.data.personal.github));
            contact_text.push_str(&format!("LinkedIn: {}", app.data.personal.linkedin));

            let profile_text = app.data.personal.profile.clone();

            let personal_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(content_area);
            let name_area = personal_chunks[0];
            let bottom_area = personal_chunks[1];

            let name_block = styled_block("Name");
            let name_paragraph = Paragraph::new(app.data.personal.name.clone())
                .block(name_block)
                .style(BODY_STYLE)
                .alignment(Alignment::Center);
            frame.render_widget(name_paragraph, name_area);

            // Bottom: split if profile exists, else full width contact
            if let Some(profile) = profile_text {
                let bottom_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(bottom_area);
                let profile_area = bottom_chunks[0];
                let contact_area = bottom_chunks[1];

                // Profile card
                let profile_block = styled_block("Profile");
                let profile_paragraph = Paragraph::new(profile)
                    .block(profile_block)
                    .style(BODY_STYLE)
                    .scroll((app.scroll_offset as u16, 0))
                    .wrap(Wrap { trim: true });
                frame.render_widget(profile_paragraph, profile_area);

                // Contact card
                let contact_block = styled_block("Contact");
                let contact_paragraph = Paragraph::new(contact_text)
                    .block(contact_block)
                    .style(BODY_STYLE)
                    .scroll((app.scroll_offset as u16, 0))
                    .wrap(Wrap { trim: true });
                frame.render_widget(contact_paragraph, contact_area);
            } else {
                // No profile – contact takes full width
                let contact_block = styled_block("Contact");
                let contact_paragraph = Paragraph::new(contact_text)
                    .block(contact_block)
                    .style(BODY_STYLE)
                    .scroll((app.scroll_offset as u16, 0))
                    .wrap(Wrap { trim: true });
                frame.render_widget(contact_paragraph, bottom_area);
            }
        }

        Section::Education => {
            let institution = format!("Institution: {}", app.data.education.institution);
            let degree = format!("Degree: {}", app.data.education.degree);
        
            let mut details = String::new();
            if let Some(gpa) = &app.data.education.gpa {
                details.push_str(&format!("GPA: {}\n", gpa));
            }
            if let Some(expected_graduation) = &app.data.education.expected_graduation {
                details.push_str(&format!("Expected Graduation: {}", expected_graduation));
            }
            // If both are missing, show something like "No details"
            if details.is_empty() {
                details.push_str("No additional details");
            }
        
            // Distinctions
            let mut distinctions = String::from("Distinctions:");
            if app.data.education.distinctions.is_empty() {
                distinctions.push_str(" None");
            } else {
                for d in &app.data.education.distinctions {
                    distinctions.push_str(&format!("\n - {}", d));
                }
            }
        
            // Layout: 3 rows 
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Length(4), Constraint::Min(0)].as_ref())
                .split(content_area);
        
            // Top: Institution
            let institution_block = styled_block("Institution");
            let institution_paragraph = Paragraph::new(institution)
                .block(institution_block)
                .style(BODY_STYLE);
            frame.render_widget(institution_paragraph, chunks[0]);
        
            // Middle: Degree (left) + Details (right)
            let middle_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[1]);
        
            // Degree
            let degree_block = styled_block("Degree");
            let degree_paragraph = Paragraph::new(degree)
                .block(degree_block)
                .style(BODY_STYLE);
            frame.render_widget(degree_paragraph, middle_chunks[0]);
        
            // Details
            let details_block = styled_block("Details");
            let details_paragraph = Paragraph::new(details)
                .block(details_block)
                .style(BODY_STYLE);
            frame.render_widget(details_paragraph, middle_chunks[1]);
        
            // Bottom: Distinctions (scrollable)
            let distinctions_block = styled_block("Distinctions");
            let distinctions_paragraph = Paragraph::new(distinctions)
                .block(distinctions_block)
                .style(BODY_STYLE)
                .scroll((app.scroll_offset as u16, 0))
                .wrap(Wrap { trim: true });
            frame.render_widget(distinctions_paragraph, chunks[2]);
        }

        Section::Skills => {
            let networking = app.data.skills.networking
                .as_ref()
                .map(|v| v.join(", "))
                .unwrap_or_default();
            let security = app.data.skills.security
                .as_ref()
                .map(|v| v.join(", "))
                .unwrap_or_default();
            let tools = app.data.skills.tools.join(", ");
            let systems = app.data.skills.systems.join(", ");
            let languages = app.data.skills.languages.join(", ");
            let spoken = app.data.skills.spoken.join(", ");
        
            let has_extra = app.data.skills.networking.is_some() || app.data.skills.security.is_some();
        
            let rows = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(content_area);
        
            if has_extra {
                // 2x3 grid: 6 panels
                let col_constraints = [
                    Constraint::Percentage(34),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ];
                let top_cols = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(col_constraints.as_ref())
                    .split(rows[0]);
                let bottom_cols = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(col_constraints.as_ref())
                    .split(rows[1]);
        
                render_card(frame, networking, "Networking", 0, top_cols[0]);
                render_card(frame, security, "Security", 0, top_cols[1]);
                render_card(frame, tools, "Tools", 0, top_cols[2]);
                render_card(frame, systems, "Systems", 0, bottom_cols[0]);
                render_card(frame, languages, "Languages", 0, bottom_cols[1]);
                render_card(frame, spoken, "Spoken", 0, bottom_cols[2]);
            } else {
                // 2x2 grid: 4 panels (original layout)
                let col_constraints = [Constraint::Percentage(50), Constraint::Percentage(50)];
                let top_cols = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(col_constraints.as_ref())
                    .split(rows[0]);
                let bottom_cols = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(col_constraints.as_ref())
                    .split(rows[1]);
        
                render_card(frame, tools, "Tools", 0, top_cols[0]);
                render_card(frame, systems, "Systems", 0, top_cols[1]);
                render_card(frame, languages, "Languages", 0, bottom_cols[0]);
                render_card(frame, spoken, "Spoken", 0, bottom_cols[1]);
            }
        }

        Section::Experience => {
            let format_item = |item: &Experience| -> (String, String) {
                let mut body = String::new();
                
                body.push_str(&format!(
                    "Role: {}\nOrganization: {}\nPeriod: {}\n",
                    item.role, item.organization, item.period
                ));
                for b in &item.bullets {
                    body.push_str(&format!("- {}\n", b));
                }
                body.push_str("\n");
                
                (item.role.clone(), body)
            };
            render_paginated::<Experience>(frame, area, "Experience", &app.data.experience, app.scroll_offset, 10, "No experience section listed", "Scroll to see more of the experience section", format_item);
        }
        
        Section::Projects => {
            let format_item = |item: &Project| -> (String, String) {
                let mut body = String::new();
                body.push_str(&format!(
                    "{} | {} | {}\n",
                    item.name,
                    item.tags.join(", "),
                    item.date
                ));

                if let Some(desc) = &item.description {
                    body.push_str(&format!("  {}\n", desc));
                }

                for b in &item.bullets {
                    body.push_str(&format!("- {}\n", b));
                }
                
                (item.name.clone(), body)
            };
            render_paginated::<Project>(frame, area, "Projects", &app.data.projects, app.scroll_offset, 10, "No projects listed", "Scroll to see more projects", format_item);
        }

        Section::Certifications => {
            let format_item = |item: &Certification| -> (String, String) {
                let body = format!(
                    "Name: {}\nIssuer: {}\nDate: {}",
                    item.name, item.issuer, item.date
                );
                
                (item.name.clone(), body)
            };
            render_paginated::<Certification>(frame, area, "Certifications", &app.data.certifications, app.scroll_offset, 4, "No certifications listed", "Scroll to see more certifications", format_item);
        }

        Section::Competitions => {
            let format_item = |item: &Competition| -> (String, String) {
                let mut body = String::new();
                body.push_str(&format!("{} | {}", item.name, item.team_name));
                if let Some(tags) = &item.tags {
                    body.push_str(&format!(" | {}", tags.join(",")));
                }
                body.push_str(&format!("\nDate: {}\n", item.date));
                for b in &item.bullets {
                    body.push_str(&format!("- {}\n", b));
                }
                
                (item.name.clone(), body)
            };
            render_paginated::<Competition>(frame, area, "Competitions", &app.data.competitions, app.scroll_offset, 8, "No competitions listed", "Scroll to see more competitions", format_item);
        }

        Section::Activities => {
            let format_item = |item: &Activity| -> (String, String) {
                let mut body = String::new();
                body.push_str(&format!("{} | {} | {}\n", item.name, item.tag, item.date));
                for b in &item.bullets {
                    body.push_str(&format!("- {}\n", b));
                }
                
                (item.name.clone(), body)
            };

            render_paginated::<Activity>(frame, area, "Activities", &app.data.activities, app.scroll_offset, 5, "No activities listed", "Scroll to see more activities", format_item);
        }
    }
}