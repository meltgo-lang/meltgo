```mycig
package main

import std::fmt

frame Animal {
    abs impl Self {
        abs func move(self)
    }
}

frame Pet : pub Animal {
    field Self {
        name: str
    }

    abs impl Self {
        abs init new(name)
        abs func greet(self)
        abs func getName(self)
    }
}

frame Cat : pub Pet {
    field Pet {
        name: str
    }

    impl Pet {
        pub init new(name) {
            name: name
        }
        pub func greet(self) {
            fmt::println("Hello! I'm {}.", self.name)
        }
        pub func getName(self) {
            self.name
        }
        pub func move(self) {
            fmt::println("{} moved.", self.name)
        }
    }
}

frame Dog : pub Pet {
    field Pet {
        *name: str
    }

    impl Pet {
        pub init new(name) {
            name: name
        }
        pub func greet(self) {
            fmt::println("Hello! I'm {}.", self.name)
        }
        pub func getName(self) {
            self.name
        }
        pub func move(self) {
            fmt::println("{} moved.", self.name)
        }
    }
}

func greet(pet: Pet) {
    pet.greet()
}

func main() {
    let cat = Cat::new("Lucy")
    let dog = Dog::new("Tom")
    
    greet(cat as Pet)
    greet(dog as pet)
}
```