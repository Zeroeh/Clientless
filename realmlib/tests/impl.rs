/* Playground for ideas */

fn main() {
    let x = Stats::Health(1);
    let f = x.matchy();
    println!("Got {}", f);
}

pub enum Stats<T> {
    Health(T),
    Mana(T),
}

impl<T> Stats<T> {
    pub fn matchy(self) -> T {
        match self {
            Stats::Health(v) => {
                println!("Health!");
                v
            }
            Stats::Mana(v) => {
                println!("Mana!");
                v
            }
        }
    }
}
