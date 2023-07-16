
fn main()
{
    enum AgeScaling {
        Kid,
        Teen,
        Adult
    }

    let cmp = |&: age: i32| -> AgeScaling {
        if age <= 12 {
            AgeScaling::Kid
        } else if age < 18 && age > 12 {
            AgeScaling::Teen
        } else {
            AgeScaling::Adult
        }
    };  

    let yourscale = cmp(17);
    if yourscale == AgeScaling::Adult {
        println!("You're allowed to watch adult videos");
    } else {
        println!("Go away");
    }
}
