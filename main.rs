Rust є мовою з високим рівнем контролю та безпечності. Нижче наведено основний приклад обробки даних в Rust, який включає в себе структури, використання векторів, функцій та методів.

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> &u8 {
        &self.age
    }

    fn birthday(&mut self) {
        self.age += 1;
    }
}

fn main() {
    let mut people = Vec::new();

    people.push(Person::new(String::from("John"), 20));
    people.push(Person::new(String::from("Jane"), 22));
    people.push(Person::new(String::from("Bob"), 25));

    for person in &people {
        println!("{} is {} years old.", person.get_name(), person.get_age());
    }

    people[0].birthday();
    people[2].birthday();

    for person in &people {
        println!("After a year {} will be {} years old.", person.get_name(), person.get_age());
    }

    let sum_of_ages: u8 = people.iter().map(|p| p.age).sum();

    println!("The total age of all people is: {}", sum_of_ages);

    let oldest_person = people.iter().max_by_key(|x| x.age);

    match oldest_person {
        Some(person) => println!("The oldest person is: {} who is {} years old.", person.get_name(), person.get_age()),
        None => println!("There are no people."),
    }

    let youngest_person = people.iter().min_by_key(|x| x.age);

    match youngest_person {
        Some(person) => println!("The youngest person is: {} who is {} years old.", person.get_name(), person.get_age()),
        None => println!("There are no people."),
    }
}
```

Цей код створює структуру Person з двома полями (name і age) та декілька методів для роботи з полями. В main() функції створюється вектор людей (people) і заповнюється декількома об'єктами Person. Код виводить інформацію про кожну людину, потім збільшує вік для деяких людей, виводить оновлену інформацію, обчислює суму віків, а також визначає найстаршу та наймолодшу людину в колекції.