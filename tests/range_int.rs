use std::collections::HashMap;
use unity_random::Random;

#[test]
fn range_int() {
    let mut random = Random::new();

    let mut map = HashMap::new();
    map.insert(0, [1900725526, 1900725046, 559298752, 107093222, 556206921]);
    map.insert(1, [1543501227, 199432971, 752298619, 138080315, 743183923]);
    map.insert(
        358118,
        [2136278644, 1595074600, 1928749762, 1103880771, 377109161],
    );
    map.insert(
        30029247,
        [607408785, 1212241089, 1349650812, 1000986081, 1024434390],
    );
    map.insert(
        719188662,
        [1596120957, 890817289, 1727690525, 42421281, 1268234803],
    );

    for (seed, values) in map {
        random.init_state(seed);

        for int in values {
            assert_eq!(int, random.range_int(0, i32::MAX));
        }
    }
}
