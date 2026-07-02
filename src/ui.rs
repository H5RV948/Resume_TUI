use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph, Wrap};
use crate::app::{App, Section};

fn render_text(frame: &mut ratatui::Frame, text: String, offset: usize, area: Rect) {
    let paragraph = Paragraph::new(text)
        .scroll((offset as u16, 0))
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

pub fn render(app: &App, frame: &mut ratatui::Frame, area: Rect) {
    let section = &app.sections[app.curr_section];
    match section {
        Section::Personal => {
            let mut text = format!(
                "Name: {}
                \nEmail: {}",
                app.data.personal.name, app.data.personal.email
            );
            
            if let Some(phone_number) = &app.data.personal.phone_number {
                text.push_str(&format!("\nPhone: {}", phone_number));
            }

            if let Some(location) = &app.data.personal.location {
                text.push_str(&format!("\nLocation: {}", location));
            }

            text.push_str(&format!("\nGitHub: {}", app.data.personal.github)); 
            text.push_str(&format!("\nLinkedIn: {}", app.data.personal.linkedin));
            
            render_text(frame, text, app.scroll_offset, area);
            
        }

        Section::Education => {
            let mut text = format!(
                "Institution: {}
                \nDegree: {}",
                app.data.education.institution, app.data.education.degree
            );

            if let Some(gpa) = &app.data.education.gpa {
                text.push_str(&format!("\nGPA: {}", gpa));
            }

            if let Some(expected_graduation) = &app.data.education.expected_graduation {
                text.push_str(&format!("\nExpected Graduation: {}", expected_graduation));
            }

            text.push_str(&format!("\nDistinctions:"));
            for d in &app.data.education.distinctions {
                
                text.push_str(&format!("\n - {}", d));
            }

            if let Some(relevant_courses) = &app.data.education.relevant_courses {
                text.push_str(&format!("\nRelevant Courses:"));
                for c in relevant_courses {
                    text.push_str(&format!("\n - {}", c));
                }
            }

            render_text(frame, text, app.scroll_offset, area);
        }

        Section::Skills => {
            let mut text = String::new();

            text.push_str(&format!("\nTools: {}", app.data.skills.tools.join(", ")));

            text.push_str(&format!("\nSystems: {}", app.data.skills.systems.join(", ")));

            text.push_str(&format!("\nLanguages: {}", app.data.skills.languages.join(", ")));

            text.push_str(&format!("\nSpoken: {}", app.data.skills.spoken.join(", ")));

            render_text(frame, text, app.scroll_offset, area);
        }

        Section::Experience => {
            let mut text = String::new();
            text.push_str(&format!("\nExperience:"));
            
            for exp in &app.data.experience {
                text.push_str(&format!("\nRole: {}", exp.role));
                text.push_str(&format!("\nOrganization: {}", exp.organization));
                text.push_str(&format!("\nPeriod: {}", exp.period));
                for b in &exp.bullets {
                    text.push_str(&format!("\n- {}", b));
                }
                text.push_str("\n"); 
           }

           render_text(frame, text, app.scroll_offset, area);
        }

        Section::Projects => {
            let mut text = String::new();
            text.push_str(&format!("\nProjects:"));

            for proj in &app.data.projects {
                text.push_str(&format!("\n{} | {} | {}", proj.name, proj.tags.join(", "), proj.date));
                
                if let Some(desc) = &proj.description {
                    text.push_str(&format!("\n  {}", desc));
                }

                for b in &proj.bullets {
                    text.push_str(&format!("\n - {}", b));
                }
                text.push_str("\n"); 
            }

            render_text(frame, text, app.scroll_offset, area);
        }

        Section::Certifications => {
            let mut text = String::new();
            text.push_str(&format!("\nCertifications:"));

            for cert in &app.data.certifications {
                text.push_str(&format!("\nName: {}", cert.name));
                text.push_str(&format!("\nIssuer: {}", cert.issuer));
                text.push_str(&format!("\nDate: {}", cert.date));
            }

            render_text(frame, text, app.scroll_offset, area);
        }

        Section::Competitions => {
            let mut text = String::new();
            text.push_str(&format!("\nCompetitions:"));

            for comp in &app.data.competitions {
                text.push_str(&format!("\n{} | {}", comp.name, comp.team_name));

                if let Some(tags) = &comp.tags {
                    text.push_str(&format!(" | {}", tags.join(",")));
                }
                
                text.push_str(&format!("\nDate: {}", comp.date));
                
                for b in &comp.bullets {
                    text.push_str(&format!("\n - {}", b));
                }
                text.push_str("\n"); 
            }

            render_text(frame, text, app.scroll_offset, area);
        }


        Section::Activities => {
            let mut text = String::new();
            text.push_str(&format!("\nActivities:"));

            for act in &app.data.activities {
                text.push_str(&format!("\n {} | {}      | {}", act.name, act.tag, act.date));
                for b in &act.bullets {
                    text.push_str(&format!("\n - {}", b));
                }
            }

            render_text(frame, text, app.scroll_offset, area);
        }

        _ => {
            let paragraph = Paragraph::new("Not yet implemented")
                .scroll((0, 0));
            frame.render_widget(paragraph, area);
        } // you're going to navigate fields that already exist
    }
}