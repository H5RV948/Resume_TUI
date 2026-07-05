use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Resume { 
    pub personal: Personal,
    
    #[serde(default)]
    pub education: Education,
    
    pub skills: Skills,
    
    #[serde(default)] // puts a default value if not specified (Optional section)
    pub experience: Vec<Experience>,
    
    pub projects: Vec<Project>,
    
    #[serde(default)]
    pub certifications: Vec<Certification>,
    
    #[serde(default)]
    pub competitions: Vec<Competition>,
    
    #[serde(default)]
    pub activities: Vec<Activity>,
}

#[derive(Deserialize)]
pub struct Personal {
    pub name: String,
    pub email: String,
    pub profile: Option<String>,
    pub phone_number: Option<String>, // returns None if not specified (Optional field)
    pub location: Option<String>,
    pub github: String,
    pub linkedin: String,
}

#[derive(Deserialize, Default, PartialEq, Eq)]
pub struct Education {
    pub institution: String,
    pub degree: String,
    pub gpa: Option<String>,
    pub expected_graduation: Option<String>,
    pub distinctions: Vec<String>,
    pub relevant_courses: Option<Vec<String>>, 
}

#[derive(Deserialize)]
pub struct Skills {
    pub tools: Vec<String>,
    pub systems: Vec<String>,
    pub languages: Vec<String>,
    pub spoken: Vec<String>,
}

#[derive(Deserialize)]
pub struct Experience {
    pub role: String,
    pub organization: String,
    pub period: String,
    pub bullets: Vec<String>,
}

#[derive(Deserialize)]
pub struct Project {
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub bullets: Vec<String>,
    pub date: String,
}

#[derive(Deserialize)]
pub struct Certification {
    pub name: String,
    pub date: String,
    pub issuer: String,
}

#[derive(Deserialize)]
pub struct Competition {
    pub name: String,
    pub team_name: String,
    pub date: String,
    pub tags: Option<Vec<String>>,
    pub bullets: Vec<String>,
}

#[derive(Deserialize)]
pub struct Activity {
    pub name: String,
    pub tag: String,
    pub bullets: Vec<String>,
    pub date: String,
}

pub fn load(path: &str) -> anyhow::Result<Resume> {
    let content = fs::read_to_string(path)?;
    let resume = toml::from_str(&content)?;
    Ok(resume)
}