mod infra;

success_tests! {
    subdir: "abclop99",
    {
        name: make_vecs_5_succ_0,
        file: "make_vecs.snek",
        input: "0",
        heap_size: 5,
        expected: "[]",
    },
    {
        name: make_vecs_5_succ_1,
        file: "make_vecs.snek",
        input: "1",
        heap_size: 5,
        expected: "[]\n[1]",
    },
    {
        name: make_vecs_5_succ_2,
        file: "make_vecs.snek",
        input: "2",
        heap_size: 5,
        expected: "[]\n[1]\n[1, 2]",
    },
    {
        name: make_vecs_5_succ_3,
        file: "make_vecs.snek",
        input: "3",
        heap_size: 5,
        expected: "[]\n[1]\n[1, 2],\n[1, 2, 3]",
    },
}

runtime_error_tests! {
    subdir: "abclop99"
    {
        name: make_vecs_5_oom_4,
        file: "make_vecs.snek",
        input: "4",
        heap_size: 5,
        expected: "out of memory",
    },
    {
        name: make_vecs_5_oom_5,
        file: "make_vecs.snek",
        input: "5",
        heap_size: 5,
        expected: "out of memory",
    },
}

static_error_tests! {
    subdir: "abclop99",
}