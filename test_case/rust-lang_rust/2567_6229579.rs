
// rustc --test test.rs && ./test
use std;
import std::map::hashmap;
import core::dvec::*;

type pets = hashmap<str, dvec<str>>;

fn broken_add(pets: pets, owner: str, name: str)
{
    alt pets.find(owner)
    {
        option::some(owns)
        {
            // owns is a copy so this is not very useful
            owns.push(name);
        }
        option::none
        {
            pets.insert(owner, dvec::from_vec([mut name]));
        }
    }
}

fn inefficient_add(pets: pets, owner: str, name: str)
{
    alt pets.find(owner)
    {
        option::some(owns)
        {
            // this does work, but there are at least two copies of a
            // potentially very large vector
            owns.push(name);
            pets.insert(owner, copy(owns));
        }
        option::none
        {
            pets.insert(owner, dvec::from_vec([mut name]));
        }
    }
}

#[test]
fn broken()
{
    let pets = std::map::str_hash();
    broken_add(pets, "Dr Evil", "Mr. Bigglesworth");
    broken_add(pets, "Dr Evil", "Sharks with frickin' laser beams");
    broken_add(pets, "Pinocchio", "Figaro");

    assert pets.size() == 2u;
    assert pets.get("Dr Evil").len() == 2u;        // this fails
    assert pets.get("Pinocchio").len() == 1u;
}

#[test]
fn inefficient()
{
    let pets = std::map::str_hash();
    inefficient_add(pets, "Dr Evil", "Mr. Bigglesworth");
    inefficient_add(pets, "Dr Evil", "Sharks with frickin' laser beams");
    inefficient_add(pets, "Pinocchio", "Figaro");

    assert pets.size() == 2u;
    assert pets.get("Dr Evil").len() == 2u;
    assert pets.get("Pinocchio").len() == 1u;
}
