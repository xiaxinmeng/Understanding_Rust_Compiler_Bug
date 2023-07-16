
extern mod std;
use core::send_map::linear::{LinearMap};

pub type Cache = LinearMap<~str, ~str>;

fn animal_noise(cache: &mut Cache, animal: ~str) -> ~str
{
    fn find_noise(animal: ~str) -> ~str
    {
        let noises = ~[(~"dog", ~"The dog barks."), (~"cat", ~"The cat meows."), (~"cow", ~"The cow moos.")];
        match do noises.position |x| {x.first_ref() == &animal}
        {
            option::Some(index) => copy *noises[index].second_ref(),
            option::None => fmt!("Don't know what %s does.", animal)
        }
    }

    match cache.find_ref(&animal)   // currently can't call this because cache is mutable
    {
            Some(noise) => copy *noise,
            None =>
            {
                let noise = find_noise(copy animal);
                cache.insert(animal, copy noise);
                noise
            }
    }
}

#[test]
fn test_noises()
{
    let mut cache = LinearMap();
    assert animal_noise(&mut cache, ~"dog") == ~"The dog barks.";
    assert animal_noise(&mut cache, ~"dog") == ~"The dog barks.";
    assert animal_noise(&mut cache, ~"cat") == ~"The cat meows.";
}
