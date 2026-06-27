pub mod resume;
pub mod app;

fn main() {
    let resume = resume::load("resume.toml").unwrap();
    println!("{}", resume.personal.name);
}
