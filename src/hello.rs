fn main() {
    use typeset::*;

    logging::console_on();

    let nodes: &[node] = &[
        box(position {x: 0.0, y: 0.0},
            size {width: 10.0, height: 10.0},
            ~"hello"
        ),
        glue(position {x: 0.0, y: 0.0},
             size {width: 10.0, height: 10.0},
             15.0,
             8.0
        )];

    io::println(nodes);
}
