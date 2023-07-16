#[derive(Debug, PartialEq)]
struct Person {
    first_name: String,
    last_name: String,
    age: usize,
}

impl TryFrom<&str> for Person {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {

        let first_name;
        let last_name;
        let age: usize;

        let mut parts = value.split(',');

        if let Some(first) = parts.next() {
            first_name = first;
        } else {
            return Err("Missing name");
        }

        if let Some(last) = parts.next() {
            last_name = last;
        } else {
            return Err("Missing last name");
        }

        if let Some(ag) = parts.next() {
             match ag.parse::<usize>() {
                Ok(val) => age = val,
                Err(e) => {
                    println!("Couldn't parse it: {} ", e);
                    return Err("Age is missing");
                }
             }
        } else {
            return Err("Missing age");
        }

        Ok(Self {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            age
        })
    }
}

impl From<&str> for Person {
    fn from(value: &str) -> Self {

        let first_name;
        let last_name;
        let age: usize;

        let mut parts = value.split(',');

        if let Some(first) = parts.next() {
            first_name = first;
        } else {
            first_name = "";
        }

        if let Some(last) = parts.next() {
            last_name = last;
        } else {
            last_name = "";
        }

        if let Some(ag) = parts.next() {
            age = ag.parse::<usize>().unwrap();
        } else {
            age = 0;
        }

        Self {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            age
        }
    }
}

fn main() {
    let p = Person::from("John,Doe,31");
    println!("The person is: {:#?}", p);

    // If you inplement From, into will be implemented automatically since internally it called: From.
    let p1: Person = "Jane,Doe,30".into();
    println!("HERE INTO: {:#?}", p1);

    let result: Result<Person, &str> = "John,D,haha".try_into();
    assert_eq!(result, Err("Age is missing"));

}
