fn main() {
    // A struct is a named grouping of fields (data belonging to the struct), which also can have methods on it:
    struct PirateShip {
        captain: String,
        crew: Vec<String>,
        treasure: f64,
    }

    // And you can have methods on the struct by using an impl block. Those methods can take a reference to self (&self) if they are just
    // reading fields, or they can use a mutable reference (&mut self) if they will be changing any data
    impl PirateShip {
        pub fn count_treasure(&self) -> f64 {
            // some code
            self.treasure
        }

        pub fn mutiny(&mut self) {
            if self.crew.len() > 0 {
                // replace the captain with one of the crew
                self.captain = self.crew.pop().unwrap();
            } else {
                println!("there's no crew to perform mutiny");
            }
        }
    }

    // To create an instance of a struct, you give the name of the struct along with a value for each of the fields, specified by name
    // (like treasure: 64.0). There is also some shorthand to use: if you have a variable in scope with the same name as one of the fields,
    // you can specify that just by name. That's confusing without an example, so let's see it in action.

    let blackbeard = "Blackbeard".to_owned();
    let crew = vec!["Scurvy".to_owned(), "Rat".to_owned(), "Polly".to_owned()];
    let ship = PirateShip {
        captain: blackbeard,
        crew,
        treasure: 64.0,
    };

    // Note that in this example, we used the method to_owned a few times. This takes a reference to a string (&str) and creates an owned
    // string (String), so that we don't have to worry about lifetimes

    struct CrewMember<'a> {
        name: &'a str,
        age: u16,
        nationality: &'a str,
    }

    impl CrewMember<'_> {
        pub fn check_age(&self) -> () {
            if self.age > 70 {
                println!("Thanks for your service, you can retire now.")
            }
        }
    }

    let seasoned_member = CrewMember {
        name: "John Smith",
        age: 71,
        nationality: "American",
    };

    seasoned_member.check_age(); //this will print "Thanks for your service, you can retire now."

    //Note that here we use <> (generics), & (references), 'a and '_ (lifetimes), instead of the to_owned() method
}
