use std::collections::HashMap;
use unity_random::Random;

#[test]
fn value() {
    let mut random = Random::new();

    let mut map = HashMap::new();
    map.insert(0, [0.5841396, 0.5840824, 0.6736069, 0.766507, 0.3050319]);
    map.insert(1, [0.9996847, 0.7742628, 0.6809838, 0.4604562, 0.5944274]);
    map.insert(
        358118,
        [0.6642595, 0.1477097, 0.9248888, 0.5928424, 0.9549153],
    );
    map.insert(
        30029247,
        [0.4087697, 0.510399, 0.8909144, 0.3268396, 0.1220958],
    );
    map.insert(
        719188662,
        [0.2724452, 0.1936961, 0.95676, 0.05701066, 0.1853699],
    );

    for (seed, values) in map {
        random.init_state(seed);

        for value in values {
            assert!((value - random.value()).abs() < f32::EPSILON);
        }
    }
}
