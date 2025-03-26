macro_rules! exchange {
    (Give $amount:literal to $name:ident) => {
        $name.add($amount);
    };
    (Take $amount:literal from $name:ident) => {
        $name.sub($amount);
    };
    (Give $amount:literal from $giver:ident to $receiver:ident) => {
        $giver.sub($amount);
        $receiver.add($amount);
    };
}

#[derive(Debug)]
struct Account {
    money: u32,
}

impl Account {
    fn add(&mut self, amount: u32) {
        self.money += amount;
    }

    fn sub(&mut self, amount: u32) {
        self.money -= amount;
    }
}

fn main() {
    let mut alice = Account { money: 100 };
    let mut bob = Account { money: 50 };

    exchange!(Give 10 to alice);
    exchange!(Take 5 from bob);
    exchange!(Give 10 from alice to bob);

    println!("Alice: {:?}", alice);
    println!("Bob: {:?}", bob);
}
