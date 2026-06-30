pub mod resume;
pub mod app;
pub mod event;

fn main() {
    let resume = resume::load("resume.toml").unwrap();
    println!("{}", resume.personal.name);
}
