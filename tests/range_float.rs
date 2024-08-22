use std::collections::HashMap;
use unity_random::Random;

#[test]
fn range_float() {
    let mut random = Random::new();

    let mut map = HashMap::new();
    map.insert(0, [0.4158604, 0.4159176, 0.3263931, 0.233493, 0.6949681]);
    map.insert(
        1,
        [0.0003153086, 0.2257372, 0.3190162, 0.5395438, 0.4055726],
    );
    map.insert(
        358118,
        [0.3357405, 0.8522903, 0.07511115, 0.4071576, 0.04508471],
    );
    map.insert(
        30029247,
        [0.5912303, 0.489601, 0.1090856, 0.6731604, 0.8779042],
    );
    map.insert(
        719188662,
        [0.7275548, 0.806304, 0.04323995, 0.9429893, 0.8146302],
    );

    for (seed, values) in map {
        random.init_state(seed);

        for float in values {
            assert!((float - random.range_float(0., 1.)).abs() < f32::EPSILON);
        }
    }
}
