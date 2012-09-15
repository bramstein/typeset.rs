struct position {
    x: float,
    y: float
}

struct size {
    width: float,
    height: float
}

enum node {
    box(position, size, ~str),
    glue(position, size, float, float),
    penalty(position, int, bool),
    element(position, size)
}
