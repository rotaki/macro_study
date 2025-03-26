macro_rules! getval {
    ($struct_type:ident) => {
        getval!($struct_type, String);
    };
    ($struct_type:ident, $return_type:ty) => {
        impl $struct_type {
            fn getval(&self) -> &$return_type {
                &self.value
            }
        }
    };
}

struct FirstName {
    value: String,
}

struct LastName {
    value: String,
}

struct Age {
    value: i32,
}

struct Pay {
    value: f64,
}

getval!(FirstName);
getval!(LastName);
getval!(Age, i32);
getval!(Pay, f64);

fn main() {
    let first_name = FirstName {
        value: "John".to_string(),
    };
    let last_name = LastName {
        value: "Doe".to_string(),
    };

    println!("First name: {}", first_name.getval());
    println!("Last name: {}", last_name.getval());

    let age = Age { value: 30 };
    let pay = Pay { value: 100.0 };

    println!("Age: {}", age.getval());
    println!("Pay: {}", pay.getval());
}
