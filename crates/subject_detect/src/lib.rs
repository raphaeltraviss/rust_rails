#[macro_use]
extern crate helix;

ruby! {
    class SubjectDetect {
        def hello() {
            println!("Hello from subject_detect!");
        }
    }
}
