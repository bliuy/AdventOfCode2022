struct Item {
    worry: i64,
}

struct Monkey {
    id: usize,
    items: Vec<Item>,
    items_inspected_count: i32,
    ops: fn(&mut Item) -> (),
    test: fn(&Item) -> usize,
}

struct Present {
    receiver: usize,
    item: Item,
}

impl Monkey {
    fn new(
        id: usize,
        starting_items: Vec<Item>,
        ops: fn(&mut Item) -> (),
        test: fn(&Item) -> usize,
    ) -> Self {
        Monkey {
            id: id,
            items: starting_items,
            items_inspected_count: 0,
            ops: ops,
            test: test,
        }
    }

    fn inspect_item(&mut self, item: &mut Item) {
        let ops_fn = self.ops;
        ops_fn(item);
        self.items_inspected_count += 1;
    }

    fn wrap_present(&self, item: Item) -> Present {
        let receiver = (self.test)(&item);
        Present {
            receiver: receiver,
            item: item,
        }
    }

    fn inspect_and_throw(&mut self) -> Vec<Present> {
        let mut presents = Vec::new();
        self.items.reverse(); // Start inspecting and throw from the start of the queue
        while let Some(mut item) = self.items.pop() {
            // Inspecting the item
            self.inspect_item(&mut item);

            // After Inspection , worry level decreases
            item.worry = item.worry.div_euclid(3);

            // Wrapping the item as a present for the other monkeys
            let present = self.wrap_present(item);
            presents.push(present);
        }
        presents
    }
}

fn main() {
    let mut monkeys = Vec::new();
    // Creating the individual monkeys
    monkeys.push(Monkey::new(
        0,
        vec![Item { worry: 61 }],
        |item| item.worry *= 11,
        |item| {
            if item.worry.rem_euclid(5) == 0 {
                7
            } else {
                4
            }
        },
    ));
    monkeys.push(Monkey::new(
        1,
        vec![
            Item { worry: 76 },
            Item { worry: 92 },
            Item { worry: 53 },
            Item { worry: 93 },
            Item { worry: 79 },
            Item { worry: 86 },
            Item { worry: 81 },
        ],
        |item| item.worry += 4,
        |item| {
            if item.worry.rem_euclid(2) == 0 {
                2
            } else {
                6
            }
        },
    ));
    monkeys.push(Monkey::new(
        2,
        vec![Item { worry: 91 }, Item { worry: 99 }],
        |item| item.worry *= 19,
        |item| {
            if item.worry.rem_euclid(13) == 0 {
                5
            } else {
                0
            }
        },
    ));
    monkeys.push(Monkey::new(
        3,
        vec![Item { worry: 58 }, Item { worry: 67 }, Item { worry: 66 }],
        |item| item.worry *= item.worry,
        |item| {
            if item.worry.rem_euclid(7) == 0 {
                6
            } else {
                1
            }
        },
    ));
    monkeys.push(Monkey::new(
        4,
        vec![
            Item { worry: 94 },
            Item { worry: 54 },
            Item { worry: 62 },
            Item { worry: 73 },
        ],
        |item| item.worry += 1,
        |item| {
            if item.worry.rem_euclid(19) == 0 {
                3
            } else {
                7
            }
        },
    ));
    monkeys.push(Monkey::new(
        5,
        vec![
            Item { worry: 59 },
            Item { worry: 95 },
            Item { worry: 51 },
            Item { worry: 58 },
            Item { worry: 58 },
        ],
        |item| item.worry += 3,
        |item| {
            if item.worry.rem_euclid(11) == 0 {
                0
            } else {
                4
            }
        },
    ));
    monkeys.push(Monkey::new(
        6,
        vec![
            Item { worry: 87 },
            Item { worry: 69 },
            Item { worry: 92 },
            Item { worry: 56 },
            Item { worry: 91 },
            Item { worry: 93 },
            Item { worry: 88 },
            Item { worry: 73 },
        ],
        |item| item.worry += 8,
        |item| {
            if item.worry.rem_euclid(3) == 0 {
                5
            } else {
                2
            }
        },
    ));
    monkeys.push(Monkey::new(
        7,
        vec![
            Item { worry: 71 },
            Item { worry: 57 },
            Item { worry: 86 },
            Item { worry: 67 },
            Item { worry: 96 },
            Item { worry: 95 },
        ],
        |item| item.worry += 7,
        |item| {
            if item.worry.rem_euclid(17) == 0 {
                3
            } else {
                1
            }
        },
    ));

    // Completing 20 rounds
    let monkey_count = monkeys.len();
    for _ in 0..20 {
        // Completing 20 rounds.
        for i in 0..monkey_count {
            let monkey_presents = monkeys.get_mut(i).unwrap().inspect_and_throw();

            for present in monkey_presents {
                let (receiver, item) = (present.receiver, present.item);
                let monkey = monkeys.get_mut(receiver).unwrap();
                monkey.items.push(item);
            }
        }
    }

    // Identifying the top counts
    let mut inspection_counts = monkeys
        .into_iter()
        .map(|monkey| monkey.items_inspected_count)
        .collect::<Vec<_>>();

    inspection_counts.sort();
    inspection_counts.reverse();

    println!("{:?}", inspection_counts);

    let first = inspection_counts.get(0).unwrap();
    let second = inspection_counts.get(1).unwrap();

    println!("Result: {}", first * second);
}
