// The input file contains the signal as a series of characters.
//
// Approach:
// Parse the input file and build a file tree from that. Then iterate
// through the file tree, and enumerate all nodes that are directories
// and have a total size of < 10000.

struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> bool>,
    test_pass_index: usize,
    test_fail_index: usize,
}

fn round(monkeys: &mut [Monkey], monkey_stats: &mut [usize]) {
    for m in 0..monkeys.len() {
        for _ in 0..monkeys[m].items.len() {
            let mut item;
            let new_monkey_m;
            { // Restrict scope of mutable monkey reference
                let monkey = &mut monkeys[m];
                // Remove the item from the front of the vector
                item = monkey.items.remove(0);
                // Update the worry level
                item = (monkey.operation)(item);

                // Relief: divide by three
                // item = item / 3;

                // Take the modulo of the product of all tests.
                item = item % (17 * 2 * 5 * 3 * 7 * 13 * 19 * 11);

                // Test the new worry level
                new_monkey_m = if (monkey.test)(item) {
                    monkey.test_pass_index
                } else {
                    monkey.test_fail_index
                };

            }

            // Throw to appropriate monkey
            monkeys[new_monkey_m].items.push(item);

            // Increment monkey stat
            monkey_stats[m] = monkey_stats[m] + 1;
        }
    }
}

fn main() {
    let mut monkeys = [
        Monkey {
            items: vec![54, 61, 97, 63, 74],
            operation: Box::new(|x| x * 7),
            test: Box::new(|x| x % 17 == 0),
            test_pass_index: 5,
            test_fail_index: 3,
        },
        Monkey {
            items: vec![61, 70, 97, 64, 99, 83, 52, 87],
            operation: Box::new(|x| x + 8),
            test: Box::new(|x| x % 2 == 0),
            test_pass_index: 7,
            test_fail_index: 6,
        },
        Monkey {
            items: vec![60, 67, 80, 65],
            operation: Box::new(|x| x * 13),
            test: Box::new(|x| x % 5 == 0),
            test_pass_index: 1,
            test_fail_index: 6,
        },
        Monkey {
            items: vec![61, 70, 76, 69, 82, 56],
            operation: Box::new(|x| x + 7),
            test: Box::new(|x| x % 3 == 0),
            test_pass_index: 5,
            test_fail_index: 2,
        },
        Monkey {
            items: vec![79, 98],
            operation: Box::new(|x| x + 2),
            test: Box::new(|x| x % 7 == 0),
            test_pass_index: 0,
            test_fail_index: 3,
        },
        Monkey {
            items: vec![72, 79, 55],
            operation: Box::new(|x| x + 1),
            test: Box::new(|x| x % 13 == 0),
            test_pass_index: 2,
            test_fail_index: 1,
        },
        Monkey {
            items: vec![63],
            operation: Box::new(|x| x + 4),
            test: Box::new(|x| x % 19 == 0),
            test_pass_index: 7,
            test_fail_index: 4,
        },
        Monkey {
            items: vec![72, 51, 93, 63, 80, 86, 81],
            operation: Box::new(|x| x * x),
            test: Box::new(|x| x % 11 == 0),
            test_pass_index: 0,
            test_fail_index: 4,
        },
    ];

    let mut monkey_stats = [0;8];

    for _ in 0..10_000 {
        round(&mut monkeys, &mut monkey_stats);
    }
    monkey_stats.sort();
    monkey_stats.reverse();
    println!("Monkey business: {}", monkey_stats[0] * monkey_stats[1]);
    println!("Monkey stats: {:?}", monkey_stats);
}
