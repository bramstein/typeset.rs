use geometry::size;
use geometry::point;

extern mod std;

/*
enum node {
    box(point, size, ~str),
    glue(point, size, float, float),
    penalty(point, int, bool),
    element(point, size)
}

let nodes: &[node] = &[
    box(point {x: 0.0, y: 0.0},
        size {width: 10.0, height: 10.0},
        ~"hello"
    ),
    glue(point {x: 0.0, y: 0.0},
         size {width: 10.0, height: 10.0},
         15.0,
         8.0
    )];
*/

struct cost {
    from: uint,
    value: uint
}

fn line_length(line: ~[~str]) -> uint {
    let mut l = 2 * (line.len() - 1);

    for line.each |word| {
        l += word.len();
    }
    return l;
}

fn compute_ratio(line: ~[~str]) -> float {
    let spaces = line.len() - 1;
    if spaces == 0 {
        return 10000.0;
    } else {
        return (60.0 - (line_length(line) as float)) / (spaces as float);
    }
}

fn set_line(line: ~[~str]) {
    let mut shortfall = 60 - line_length(line);

    for line.eachi |index, word| {
        if index != line.len() - 1 {
            io::print(word);
            if shortfall > 0 {
                io::print("   ");
                shortfall -= 1;
            } else if shortfall < 0 {
                io::print(" ");
                shortfall += 1;
            } else {
                io::print("  ");
            }
        } else {
            io::print(word);
        }
    }
    io::print(~"  " + float::to_str(compute_ratio(line), 3) + ~"\n");
}

fn set_last_line(line: ~[~str]) {
    for line.eachi |index, word| {
        if index != line.len() - 1 {
            io::print(word + "  ");
        } else {
            io::print(word);
        }
    }
    io::print("\n");
}

fn set_paragraph(para: ~[~[~str]]) {
    for para.eachi |index, line| {
        if index != para.len() - 1 {
            set_line(line);
        } else {
            set_last_line(line);
        }
    }
}

fn main() {
    let input = ~"You may never have thought of it, but fonts (better: typefaces) usually have a mathematical definition somehow. If a font is given as bitmap, this is often a result originating from a more compact description. Imagine the situation that you have bitmaps at 300dpi, and you buy a 600dpi printer. It wouldn’t look pretty. There is then a need for a mathematical way of describing arbitrary shapes. These shapes can also be three-dimensional; in fact, a lot of the mathematics in this chapter was developed by a car manufacturer for modeling car body shapes. But let us for now only look in two dimensions, which means that the curves are lines, rather than planes.";
    let paragraph = input.split_char(' ');
    let mut costs: ~[~cost] = paragraph.map(|w| {
        ~cost { value: 0, from: 0 }
    });
    let mut active: ~[uint] = ~[0];

    costs[0] = ~cost { from: -1, value: 10000 };

    for paragraph.eachi |w, word| {
        active = active.filter(|a| {
            let line = paragraph.slice(a, w);
            let mut ratio = compute_ratio(line);

            if w == paragraph.len() - 1 && ratio > 0.0 {
                ratio = 0.0;
            }

            if ratio < -1.0 {
                false;
            }

            // update costs
            if a > 0 && costs[a - 1].value < 10000 {
                let mut to_here;

                if ratio <= 1.0 && ratio >= -1.0 {
                    to_here = (float::abs(ratio) as uint);
                } else {
                    to_here = 10000;
                }
                if costs[w].value == 0 || to_here < costs[w].value {
                    costs[w] = ~cost { value: to_here, from: a - 1 };
                }
            }
            true
        });
        active = active + ~[w];
    }

    let mut cur = paragraph.len() - 1;
    let mut broken: ~[~[~str]] = ~[];

    while cur != -1 {
        let prev = costs[cur].from;
        let line = paragraph.slice(prev + 1, cur + 1);
        broken = ~[line] + broken;
        cur = prev;
    }

    set_paragraph(broken);
}
