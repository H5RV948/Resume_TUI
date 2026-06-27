use crate::resume::Resume;

pub enum Section {
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
    pub section: Section,
}

impl App {
    pub fn new(resume: Resume) -> App{
        App {   
            running: true,
            data: resume,
            section: Section::Personal,
        }
    }
}   