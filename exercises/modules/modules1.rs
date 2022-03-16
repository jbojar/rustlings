// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)



mod sausage_factory {
<<<<<<< HEAD
    pub fn make_sausage() {
=======
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        get_secret_recipe();
>>>>>>> 4.6.0
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
