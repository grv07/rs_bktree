use std::cmp::min;

#[allow(dead_code)]
fn main() {
    // let lev = lev_basic("DatabasesesGame", "FLoatIntRef");
    let lev = lev("DatabasesesGame", "DatabasesesGame");

    println!("Distance between is {}", lev);
}

#[allow(dead_code)]
fn lev_basic(a: &str, b: &str) -> usize {
    println!("Compare: {a} {b}");

    if a.len() == 0 {
        return b.len();
    }

    if b.len() == 0 {
        return a.len();
    }

    let (a_head, a_tail) = a.split_at(1);
    let (b_head, b_tail) = b.split_at(1);

    // println!("{a_head} {a_tail}");
    // println!("{b_head} {b_tail}");

    if a_head == b_head {
        return lev(a_tail, b_tail);
    }

    1 + std::cmp::min(
        lev(a_tail, b),
        std::cmp::min(lev(a, b_tail), lev(a_tail, b_tail)),
    )
}

#[allow(dead_code)]
fn dump(m: &Vec<Vec<usize>>) {
    for r in m {
        for c in r {
            print!(" {:<2}", c);
        }
        println!("");
    }
}

pub fn lev(a: &str, b: &str) -> usize {
    // println!("{a} {b}");
    let row = b.len() + 1;
    let col = a.len() + 1;

    let mut m = vec![vec![0; col]; row];

    // set first column
    for i in 1..col {
        m[0][i] = i;
    }

    // set first row
    for j in 1..row {
        m[j][0] = j;
    }

    let a = a.chars().collect::<Vec<char>>();
    let b = b.chars().collect::<Vec<char>>();

    for r in 1..row {
        let r = r as usize;
        for c in 1..col {
            let c = c as usize;
            let (ac, bc) = (b[r - 1], a[c - 1]);
            if ac == bc {
                m[r][c] = m[r - 1][c - 1];
                continue;
            }

            m[r][c] = 1 + min(m[r][c - 1], min(m[r - 1][c], m[r - 1][c - 1]));
        }
    }

    // dump(&m);

    m[row - 1][col - 1]
}
