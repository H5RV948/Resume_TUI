use crate::resume::*;
use crate::event::Action;

#[derive(PartialEq, Eq)] // allows to compare with the enum types
pub enum Section { // for doing a match and avoiding having 2 sections at the same time
    Personal,
    Education,
    Skills,
    Projects,
    Experience,
    Certifications,
    Competitions,
    LeadershipActivities,
}

pub struct App {
    pub running: bool,
    pub data: Resume,
    pub sections: Vec<Section>,
    pub curr_section: usize,
    pub scroll_offset: usize,
}

impl App {
    pub fn new(resume: Resume) -> App { // data is loaded in main
        let mut sections = Vec::new();

        sections.push(Section::Personal);
        
        if resume.education != Education::default() {
            sections.push(Section::Education);
        };

        sections.push(Section::Skills);

        sections.push(Section::Projects);

        if !resume.experience.is_empty() {
            sections.push(Section::Experience);
        };

        if !resume.certifications.is_empty() {
            sections.push(Section::Certifications);
        };

        if !resume.competitions.is_empty() {
            sections.push(Section::Competitions);
        };

        if !resume.leadership_activities.is_empty() {
            sections.push(Section::LeadershipActivities);
        };
        
        App {   
            running: true,
            data: resume,
            sections,
            curr_section: 0, // I want to always start on Personal section; the first section to enter on the vec
            scroll_offset: 0,
        }
    }

    pub fn update(&mut self, action: Action) {
        match action {
            Action::Up => {
                if self.scroll_offset != 0 {
                    self.scroll_offset -= 1
                }
            }

            Action::Down => {
                self.scroll_offset += 1
            }

            Action::Right => {
                if self.curr_section != self.sections.len()-1 {
                    self.scroll_offset = 0;
                    self.curr_section += 1
                }
            }
            
            Action::Left => {
                if self.curr_section != 0 {
                    self.scroll_offset = 0;
                    self.curr_section -= 1
                }
            }     

            Action::Quit => {
                self.running = false
            }

            Action::Resize(_,_) => ()
            
        }
    }
}
