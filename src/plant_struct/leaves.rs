pub struct Tree {
    pub name: String,

    pub age: u32,

}
#[derive(Debug)]
pub struct Forest<T> {
    pub name: String,

    pub trees: Vec<T>,
}

impl<T> Forest<T>
    where T: Default + Clone
{
    pub fn new() -> Forest<T> {
        Forest { name: "".to_string(), trees: vec![T::default(); 10] }
    }
}


impl Tree {
    pub fn fruit(self: &Self) {
        println!("{} is delicious!", self.name)
    }

    pub fn new() -> Tree {
        Tree { name: "".to_string(), age: 0 }
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.age == other.age
    }
}

impl Eq for Tree {}



