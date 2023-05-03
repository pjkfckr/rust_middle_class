#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cake {
    Chocolate,
    MapleBacon,
    Spice,
}

#[derive(Debug)]
pub struct Party {
    pub at_restaurant: bool,
    pub num_people: u8,
    pub cake: Cake,
}

impl Default for Party {
    fn default() -> Self {
        Self {
            at_restaurant: true,
            num_people: 8,
            cake: Cake::Chocolate,
        }
    }
}

impl PartialEq for Party {
    fn eq(&self, other: &Self) -> bool {
        self.cake == other.cake
    }
}

impl From<&Party> for Cake {
    fn from(value: &Party) -> Self {
        value.cake.clone()
    }
}

fn main() {
    // 1. The code below doesn't work because Cake doesn't implement Debug.
    // - Derive the debug trait for the Cake enum above so this code will work. run the code
    // Debug 를 Cake 에 추가
    let cake = Cake::Spice;
    admire_cake(cake);

    // 2. Uncomment the code below. It doesn't work since `cake` was *moved* into the admire_cake()
    // function. Let's fix the Cake enum so the code below works without any changes.
    // - Derive the Copy trait for the Cake enm so that `cake` gets copied into the admire_cake()
    // function instead of moved.
    // - Hint: You may need to derive another trait in order to be able to derive the Copy trait

    match cake {
        Cake::Chocolate => println!("The name's Chocolate. Dark...Chocolate"),
        Cake::MapleBacon => println!("Dreams do come true"),
        Cake::Spice => println!("Great, let's spice it up!"),
    }

    // 3. Uncomment the println below. It doesn't work since the Part struct doesn't implement the Debug or Default traits
    // - Derive the Debug trait for the Part struct
    // - Manually implement the Default trait for the Party struct.
    //   Use the value below as the default value that you return from the `default` method
    // Party {
    //     at_restaurant: true,
    //     num_people: 8,
    //     cake: Cake::Chocolate
    // }
    println!("{:?}", Party::default());

    // 4. You prefer Maple Bacon cake.
    //    Use "struct update syntax" to create a Dessert with `cake`
    //    set to `Cake::MapleBacon`, but the rest of values are default.
    // Hint: The trick to struct update syntax is specifying the value(s)
    //       you want to customize first and then ending the struct with `..Default::default()`
    //       but no comma after that!

    let party = Party {
        cake: Cake::MapleBacon,
        ..Default::default()
    };
    println!("Yes! My party has my favorite {:?} cake!", party.cake);

    // 5. Parties are "equal" if they have the same cake.
    // - Derive the PartialEq trait for the Cake enum so Cakes can be compared.
    // - Manually implement the PartialEq trait for Party. If different parties have the same cake,
    //   then they are equal, no matter the location or number of attendees at the party.
    // - Uncomment and run the code below.
    let other_party = Party {
        at_restaurant: false,
        num_people: 235,
        cake: Cake::MapleBacon,
    };
    if party == other_party {
        println!("Your party is just like mine!");
    }

    // Challenge: You would like to be able to pass a Party struct into the smell_cake() function
    // which takes a type T which implements the Into<Cake> trait.
    // - Uncomment the code below AND uncomment the smell_cake() function at the bottom of this file
    // - Implement `From<Party> for Cake` so that the function call below works.

    smell_cake(&party.into());
}

pub fn admire_cake(cake: Cake) {
    println!("Wat a nice {:?} cake!", cake);
}

pub fn smell_cake<T: Into<Cake>>(s: T) {
    println!("Hmm...something smells like a {:?} cake!", s.into());
}
