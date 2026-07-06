use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Paragraph, Wrap},
    Frame,
};
use crate::app::{App, Section};

pub fn centered_rect(area: Rect, max_width: u16, max_height: u16) -> Rect {
    let width = area.width.min(max_width);
    let height = area.height.min(max_height);
    let x = (area.width - width) / 2;
    let y = (area.height - height) / 2;
    Rect::new(x, y, width, height)
}

/// Render a single card (bordered block with a title and scrollable text)
fn render_card(frame: &mut Frame, text: String, title: &str, offset: usize, area: Rect) {
    let block = Block::bordered()
        .title(title)
        .border_style(Style::default().fg(Color::Gray))
        .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(Color::White))
        .scroll((offset as u16, 0))
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

/// Render the footer with navigation hints
fn render_footer(frame: &mut Frame, area: Rect) {
    let text = "[← →] Sections  [↑ ↓] Scroll  [q/Esc] Quit";
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::Gray).bg(Color::DarkGray))
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
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
            // --- Contact text ---
            let mut contact_text = format!("Email: {}\n", app.data.personal.email);
            if let Some(phone) = &app.data.personal.phone_number {
                contact_text.push_str(&format!("Phone: {}\n", phone));  
            }
            if let Some(location) = &app.data.personal.location {
                contact_text.push_str(&format!("Location: {}\n", location));
            }
            contact_text.push_str(&format!("GitHub: {}\n", app.data.personal.github));
            contact_text.push_str(&format!("LinkedIn: {}", app.data.personal.linkedin));

            // --- Profile text (optional) ---
            let profile_text = app.data.personal.profile.clone();

            // Layout inside Personal: top row (Name), bottom row (split or full)
            let personal_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(content_area);
            let name_area = personal_chunks[0];
            let bottom_area = personal_chunks[1];

            // Name card
            let name_block = Block::bordered()
                .title("Name")
                .border_style(Style::default().fg(Color::Gray))
                .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
            let name_paragraph = Paragraph::new(app.data.personal.name.clone())
                .block(name_block)
                .style(Style::default().fg(Color::White))
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
                let profile_block = Block::bordered()
                    .title("Profile")
                    .border_style(Style::default().fg(Color::Gray))
                    .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                let profile_paragraph = Paragraph::new(profile)
                    .block(profile_block)
                    .style(Style::default().fg(Color::White))
                    .scroll((app.scroll_offset as u16, 0))
                    .wrap(Wrap { trim: true });
                frame.render_widget(profile_paragraph, profile_area);

                // Contact card
                let contact_block = Block::bordered()
                    .title("Contact")
                    .border_style(Style::default().fg(Color::Gray))
                    .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                let contact_paragraph = Paragraph::new(contact_text)
                    .block(contact_block)
                    .style(Style::default().fg(Color::White))
                    .scroll((app.scroll_offset as u16, 0))
                    .wrap(Wrap { trim: true });
                frame.render_widget(contact_paragraph, contact_area);
            } else {
                // No profile – contact takes full width
                let contact_block = Block::bordered()
                    .title("Contact")
                    .border_style(Style::default().fg(Color::Gray))
                    .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                let contact_paragraph = Paragraph::new(contact_text)
                    .block(contact_block)
                    .style(Style::default().fg(Color::White))
                    .scroll((app.scroll_offset as u16, 0))
                    .wrap(Wrap { trim: true });
                frame.render_widget(contact_paragraph, bottom_area);
            }
        }

        Section::Education => {
            // --- Build parts ---
            let institution = format!("Institution: {}", app.data.education.institution);
            let degree = format!("Degree: {}", app.data.education.degree);
        
            // Details: GPA and Expected Graduation (each on its own line)
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
        
            // --- Layout: 3 rows ---
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Length(4), Constraint::Min(0)].as_ref())
                .split(content_area);
        
            // Top: Institution
            let institution_block = Block::bordered()
                .title("Institution")
                .border_style(Style::default().fg(Color::Gray))
                .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
            let institution_paragraph = Paragraph::new(institution)
                .block(institution_block)
                .style(Style::default().fg(Color::White));
            frame.render_widget(institution_paragraph, chunks[0]);
        
            // Middle: Degree (left) + Details (right)
            let middle_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[1]);
        
            // Degree
            let degree_block = Block::bordered()
                .title("Degree")
                .border_style(Style::default().fg(Color::Gray))
                .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
            let degree_paragraph = Paragraph::new(degree)
                .block(degree_block)
                .style(Style::default().fg(Color::White));
            frame.render_widget(degree_paragraph, middle_chunks[0]);
        
            // Details
            let details_block = Block::bordered()
                .title("Details")
                .border_style(Style::default().fg(Color::Gray))
                .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
            let details_paragraph = Paragraph::new(details)
                .block(details_block)
                .style(Style::default().fg(Color::White));
            frame.render_widget(details_paragraph, middle_chunks[1]);
        
            // Bottom: Distinctions (scrollable)
            let distinctions_block = Block::bordered()
                .title("Distinctions")
                .border_style(Style::default().fg(Color::Gray))
                .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
            let distinctions_paragraph = Paragraph::new(distinctions)
                .block(distinctions_block)
                .style(Style::default().fg(Color::White))
                .scroll((app.scroll_offset as u16, 0))
                .wrap(Wrap { trim: true });
            frame.render_widget(distinctions_paragraph, chunks[2]);
        }

        Section::Skills => {
            // --- Build the four text strings ---
            let tools = app.data.skills.tools.join(", ");
            let systems = app.data.skills.systems.join(", ");
            let languages = app.data.skills.languages.join(", ");
            let spoken = app.data.skills.spoken.join(", ");
        
            // --- Split content_area into 2 rows ---
            let rows = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(content_area);
        
            // --- Split each row into 2 columns ---
            let top_cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(rows[0]);
        
            let bottom_cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(rows[1]);
        
            // --- Helper to render a single square ---
            let render_square = |frame: &mut Frame, text: String, title: &str, area: Rect| {
                let block = Block::bordered()
                    .title(title)
                    .border_style(Style::default().fg(Color::Gray))
                    .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                let paragraph = Paragraph::new(text)
                    .block(block)
                    .style(Style::default().fg(Color::White))
                    .wrap(Wrap { trim: true });
                frame.render_widget(paragraph, area);
            };
        
            // --- Render each square ---
            render_square(frame, tools, "Tools", top_cols[0]);
            render_square(frame, systems, "Systems", top_cols[1]);
            render_square(frame, languages, "Languages", bottom_cols[0]);
            render_square(frame, spoken, "Spoken", bottom_cols[1]);
        }

        Section::Experience => {
            let mut text = String::new();
            for exp in &app.data.experience {
                text.push_str(&format!(
                    "Role: {}\nOrganization: {}\nPeriod: {}\n",
                    exp.role, exp.organization, exp.period
                ));
                for b in &exp.bullets {
                    text.push_str(&format!("- {}\n", b));
                }
                text.push_str("\n");
            }
            render_card(frame, text, "Experience", app.scroll_offset, content_area);
        }
        
        Section::Projects => {
            let projects = &app.data.projects;
            if projects.is_empty() {
                // If no projects, show a placeholder
                let block = Block::bordered()
                    .title("Projects")
                    .border_style(Style::default().fg(Color::Gray))
                    .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                let placeholder = Paragraph::new("No projects listed")
                    .block(block)
                    .style(Style::default().fg(Color::White));
                frame.render_widget(placeholder, content_area);
                return;
            }
        
            // Determine how many projects fit vertically
            // Each card will have a fixed height of 5 lines (including border).
            // If your terminal is small, you may adjust this.
            const CARD_HEIGHT: usize = 10;
            let available_height = content_area.height as usize;
            let max_visible = available_height / CARD_HEIGHT;
            let total = projects.len();
        
            // Clamp scroll_offset so we don't scroll past the last visible project
            let max_scroll = total.saturating_sub(max_visible);
            let offset = app.scroll_offset.min(max_scroll);
        
            // Visible range
            let start = offset;
            let end = (start + max_visible).min(total);
        
            // Build a layout with one row per visible project
            let mut constraints = Vec::new();
            for _ in start..end {
                constraints.push(Constraint::Length(CARD_HEIGHT as u16));
            }
            // Fill remaining space with a flexible constraint (optional)
            if constraints.is_empty() {
                // No projects visible – show a placeholder
                let block = Block::bordered()
                    .title("Projects")
                    .border_style(Style::default().fg(Color::Gray))
                    .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                let placeholder = Paragraph::new("Scroll to see more projects")
                    .block(block)
                    .style(Style::default().fg(Color::White));
                frame.render_widget(placeholder, content_area);
                return;
            }
        
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints)
                .split(content_area);
        
            // Render each visible project
            for (i, chunk) in chunks.iter().enumerate() {
                let proj = &projects[start + i];
                let mut text = String::new();
                // Title line: name, tags, date
                text.push_str(&format!(
                    "{} | {} | {}\n",
                    proj.name,
                    proj.tags.join(", "),
                    proj.date
                ));
                // Description
                if let Some(desc) = &proj.description {
                    text.push_str(&format!("  {}\n", desc));
                }
                // Bullets
                for b in &proj.bullets {
                    text.push_str(&format!("- {}\n", b));
                }
        
                let block = Block::bordered()
                    .title(proj.name.clone())
                    .border_style(Style::default().fg(Color::Gray))
                    .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                let paragraph = Paragraph::new(text)
                    .block(block)
                    .style(Style::default().fg(Color::White))
                    .wrap(Wrap { trim: true });
                frame.render_widget(paragraph, *chunk);
            }
        }

        Section::Certifications => {
            let certs = &app.data.certifications;
            if certs.is_empty() {
                let block = Block::bordered()
                    .title("Certifications")
                    .border_style(Style::default().fg(Color::Gray))
                    .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                let placeholder = Paragraph::new("No certifications listed")
                    .block(block)
                    .style(Style::default().fg(Color::White));
                frame.render_widget(placeholder, content_area);
                return;
            }
        
            const CARD_HEIGHT: usize = 4; // enough for Name, Issuer, Date, and a blank line
            let available_height = content_area.height as usize;
            let max_visible = available_height / CARD_HEIGHT;
            let total = certs.len();
        
            let max_scroll = total.saturating_sub(max_visible);
            let offset = app.scroll_offset.min(max_scroll);
            let start = offset;
            let end = (start + max_visible).min(total);
        
            // Build constraints for visible cards
            let mut constraints = Vec::new();
            for _ in start..end {
                constraints.push(Constraint::Length(CARD_HEIGHT as u16));
            }
            if constraints.is_empty() {
                let block = Block::bordered()
                    .title("Certifications")
                    .border_style(Style::default().fg(Color::Gray))
                    .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                let placeholder = Paragraph::new("Scroll to see more certifications")
                    .block(block)
                    .style(Style::default().fg(Color::White));
                frame.render_widget(placeholder, content_area);
                return;
            }
        
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints)
                .split(content_area);
        
            for (i, chunk) in chunks.iter().enumerate() {
                let cert = &certs[start + i];
                let text = format!(
                    "Name: {}\nIssuer: {}\nDate: {}",
                    cert.name, cert.issuer, cert.date
                );
                let block = Block::bordered()
                    .title(cert.name.clone())
                    .border_style(Style::default().fg(Color::Gray))
                    .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                let paragraph = Paragraph::new(text)
                    .block(block)
                    .style(Style::default().fg(Color::White))
                    .wrap(Wrap { trim: true });
                frame.render_widget(paragraph, *chunk);
            }
        }

        Section::Competitions => {
            let comps = &app.data.competitions;
            if comps.is_empty() {
                let block = Block::bordered()
                    .title("Competitions")
                    .border_style(Style::default().fg(Color::Gray))
                    .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                let placeholder = Paragraph::new("No competitions listed")
                    .block(block)
                    .style(Style::default().fg(Color::White));
                frame.render_widget(placeholder, content_area);
                return;
            }
        
            const CARD_HEIGHT: usize = 8; // Adjust based on typical content (title, team, date, bullets)
            let available_height = content_area.height as usize;
            let max_visible = available_height / CARD_HEIGHT;
            let total = comps.len();
        
            let max_scroll = total.saturating_sub(max_visible);
            let offset = app.scroll_offset.min(max_scroll);
            let start = offset;
            let end = (start + max_visible).min(total);
        
            let mut constraints = Vec::new();
            for _ in start..end {
                constraints.push(Constraint::Length(CARD_HEIGHT as u16));
            }
            if constraints.is_empty() {
                let block = Block::bordered()
                    .title("Competitions")
                    .border_style(Style::default().fg(Color::Gray))
                    .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                let placeholder = Paragraph::new("Scroll to see more competitions")
                    .block(block)
                    .style(Style::default().fg(Color::White));
                frame.render_widget(placeholder, content_area);
                return;
            }
        
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints)
                .split(content_area);
        
            for (i, chunk) in chunks.iter().enumerate() {
                let comp = &comps[start + i];
                let mut text = String::new();
                text.push_str(&format!("{} | {}", comp.name, comp.team_name));
                if let Some(tags) = &comp.tags {
                    text.push_str(&format!(" | {}", tags.join(",")));
                }
                text.push_str(&format!("\nDate: {}\n", comp.date));
                for b in &comp.bullets {
                    text.push_str(&format!("- {}\n", b));
                }
        
                let block = Block::bordered()
                    .title(comp.name.clone())
                    .border_style(Style::default().fg(Color::Gray))
                    .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                let paragraph = Paragraph::new(text)
                    .block(block)
                    .style(Style::default().fg(Color::White))
                    .wrap(Wrap { trim: true });
                frame.render_widget(paragraph, *chunk);
            }
        }

        Section::Activities => {
            let acts = &app.data.activities;
            if acts.is_empty() {
                let block = Block::bordered()
                    .title("Activities")
                    .border_style(Style::default().fg(Color::Gray))
                    .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                let placeholder = Paragraph::new("No activities listed")
                    .block(block)
                    .style(Style::default().fg(Color::White));
                frame.render_widget(placeholder, content_area);
                return;
            }
        
            const CARD_HEIGHT: usize = 5; // Adjust based on typical content (name, tag, date, bullets)
            let available_height = content_area.height as usize;
            let max_visible = available_height / CARD_HEIGHT;
            let total = acts.len();
        
            let max_scroll = total.saturating_sub(max_visible);
            let offset = app.scroll_offset.min(max_scroll);
            let start = offset;
            let end = (start + max_visible).min(total);
        
            let mut constraints = Vec::new();
            for _ in start..end {
                constraints.push(Constraint::Length(CARD_HEIGHT as u16));
            }
            if constraints.is_empty() {
                let block = Block::bordered()
                    .title("Activities")
                    .border_style(Style::default().fg(Color::Gray))
                    .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                let placeholder = Paragraph::new("Scroll to see more activities")
                    .block(block)
                    .style(Style::default().fg(Color::White));
                frame.render_widget(placeholder, content_area);
                return;
            }
        
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints)
                .split(content_area);
        
            for (i, chunk) in chunks.iter().enumerate() {
                let act = &acts[start + i];
                let mut text = String::new();
                text.push_str(&format!("{} | {} | {}\n", act.name, act.tag, act.date));
                for b in &act.bullets {
                    text.push_str(&format!("- {}\n", b));
                }
        
                let block = Block::bordered()
                    .title(act.name.clone())
                    .border_style(Style::default().fg(Color::Gray))
                    .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                let paragraph = Paragraph::new(text)
                    .block(block)
                    .style(Style::default().fg(Color::White))
                    .wrap(Wrap { trim: true });
                frame.render_widget(paragraph, *chunk);
            }
        }
    }
}