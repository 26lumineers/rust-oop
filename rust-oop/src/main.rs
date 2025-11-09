// trait Weapon {
//     fn attack(&self);
// }

// struct Sword;
// impl Weapon for Sword {
//     fn attack(&self) {
//         println!("Swinging a sword!");
//     }
// }
// struct Staff;
// impl Weapon for Staff {
//     fn attack(&self) {
//         println!("Casting a spell with the staff!");
//     }
// }
// struct Bow;

// impl Weapon for Bow {
//     fn attack(&self) {
//         println!("Shooting an arrow with the bow!");
//     }
// }
// struct Warrior {
//     health: u8,
//     strength: u8,
//     intelligence: u8,
//     weapon: Box<dyn Weapon>,
// }
// impl Warrior {
//     fn new() -> Self {
//         Self {
//             health: 100,
//             strength: 80,
//             intelligence: 30,
//             weapon: Box::new(Sword {}),
//         }
//     }
//     fn health_increase(&mut self, amount: u8) {
//         if self.health + amount > 100 {
//             self.health = 100;
//         } else {
//             self.health += amount;
//         }
//     }
//     fn health_decrease(&mut self, amount: u8) {
//         self.health = self.health.saturating_sub(amount);
//     }
// }
// struct Mage {
//     health: u8,
//     strength: u8,
//     intelligence: u8,
//     weapon: Box<dyn Weapon>,
// }
// impl Mage {
//     fn new() -> Self {
//         Self {
//             health: 100,
//             strength: 10,
//             intelligence: 50,
//             weapon: Box::new(Staff {}),
//         }
//     }
//     fn health_increase(&mut self, amount: u8) {
//         if self.health + amount > 100 {
//             self.health = 100;
//         } else {
//             self.health += amount;
//         }
//     }
//     fn health_decrease(&mut self, amount: u8) {
//         self.health = self.health.saturating_sub(amount);
//     }
// }

// struct Ranger {
//     health: u8,
//     strength: u8,
//     intelligence: u8,
//     weapon: Box<dyn Weapon>,
// }
// impl Ranger {
//     fn new() -> Self {
//         Self {
//             health: 100,
//             strength: 20,
//             intelligence: 30,
//             weapon: Box::new(Bow {}),
//         }
//     }
//     fn health_increase(&mut self, amount: u8) {
//         if self.health + amount > 100 {
//             self.health = 100;
//         } else {
//             self.health += amount;
//         }
//     }
//     fn health_decrease(&mut self, amount: u8) {
//         self.health = self.health.saturating_sub(amount);
//     }
// }

// fn speal_attack(weapon: Box<dyn Weapon>) {
//     weapon.attack();
// }

// fn main() {
//     let mut warrior = Warrior::new();
//     let mut mage = Mage::new();
//     let mut ranger = Ranger::new();

//     warrior.health_decrease(10);
//     mage.health_decrease(10);
//     ranger.health_decrease(10);

//     warrior.health_increase(20);
//     mage.health_increase(20);
//     ranger.health_increase(20);

//     speal_attack(warrior.weapon);
//     speal_attack(mage.weapon);
//     speal_attack(ranger.weapon);
// }

//Solid principles example
//old
// struct QuestNotifier;
// impl QuestNotifier {
//     fn notify(&self, message: &str) {
//         println!("Quest Notification: {}", message);
//     }
// }

// struct QuestManager {
//     notifier: QuestNotifier,
// }

// impl QuestManager {
//     fn complete_quest(&self) {
//         self.notifier.notify("Quest '{}' completed!");
//     }
// }

//after refactor

trait QuestNotifier {
    fn notify(&self, message: &str);
}

struct Pigeon;
impl QuestNotifier for Pigeon {
    fn notify(&self, message: &str) {
        println!("Pigeon Notification: {}", message);
    }
}
struct Email;
impl QuestNotifier for Email {
    fn notify(&self, message: &str) {
        println!("Email Notification: {}", message);
    }
}

struct QuestManager;

impl QuestManager {
    fn complete_quest<T: QuestNotifier>(&self, notifier: T) {
        notifier.notify("Quest '{}' completed!");
    }
}
fn main() {
    let quest_manger = QuestManager;
    let pg_notifier = Pigeon;
    let email_notifier = Email;

    quest_manger.complete_quest(pg_notifier);
    quest_manger.complete_quest(email_notifier);
}
