
import std::map::hashmap; 

type namespace = {prefix: str, path: str};

class store
{
    new(namespaces: [namespace])
    {
        self.namespaces = [];
        self.namespaces += namespaces;
    }

    priv
    {
        let mut namespaces: [namespace];
    }
}
