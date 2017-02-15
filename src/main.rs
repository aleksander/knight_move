extern crate libc;

fn print_array (arr: &[[usize; 10]; 10]) {
    for y in 0..10 {
        for x in 0..10 {
            print!("{:2} ", arr[x][y]);
        }
        println!();
    }
}

fn have_isolated_points (arr: &[[usize; 10]; 10]) -> bool {
    for y in 0..10 {
        for x in 0..10 {
            if arr[x][y] == 0 {
                let mut free_neighbors = 0;
                for i in 0..MOVES.len() {
                    let (dx,dy) = MOVES[i];
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && nx <= 9 && ny >= 0 && ny <= 9 && arr[nx as usize][ny as usize] == 0 {
                        free_neighbors += 1;
                    }
                }
                if free_neighbors == 0 { return true; }
            }
        }
    }
    false
}

const MOVES: [(isize,isize); 8] = [(1,2), (2,1), (2,-1), (1,-2), (-2,-1), (-2,1), (-1,-2), (-1,2)];

fn main() {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut c: usize = 0;
    let mut maxc: usize = 0;
    let mut f: [[usize; 10]; 10] = [[0; 10]; 10];
    let mut neighbors: [[Vec<(usize,usize)>; 10]; 10] = [
        [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
    ];
    for x in 0..10 {
        for y in 0..10 {
            for i in 0..MOVES.len() {
                let (dx,dy) = MOVES[i];
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx <= 9 && ny >= 0 && ny <= 9 {
                    neighbors[x][y].push((nx as usize, ny as usize));
                }
            }
        }
    }
    let mut neighbors_count: [[usize; 10]; 10] = [[0; 10]; 10];
    for x in 0..10 {
        for y in 0..10 {
            neighbors_count[x][y] = neighbors[x][y].len();
            print!("{} ", neighbors_count[x][y]);
        }
        println!();
    }
    let mut i = 0;
    let mut stack: [(usize,isize,isize); 101] = [(0,0,0); 101];

    loop {
        //use std::time;
        //let start = time::Instant::now();
        //for _ in 0..1_000_000_000 {
            if i < MOVES.len() {
                //print!("{}: try move {}: ", c, i);
                let (dx,dy) = MOVES[i];
                let nx = x + dx;
                let ny = y + dy;
                let can_move = nx >= 0 && nx <= 9 && ny >= 0 && ny <= 9 && f[nx as usize][ny as usize] == 0;
                if can_move {
                    // save current position to stack
                    stack[c as usize] = (i,x,y);
                    // save current position to field
                    f[x as usize][y as usize] = c as usize + 1;
                    // do move
                    c += 1;
                    //println!("move ({},{}) -> ({},{})", x, y, nx, ny);
                    //print_array(&f);
                    x = nx;
                    y = ny;
                    if c == 99 {
                        println!("solution is found!");
                        print_array(&f);
                    }
                    // reset move counter
                    i = 0;
                    if have_isolated_points(&f) {
                        //println!("{}: isolated points!", c);
                        //print_array(&f);
                        //unsafe { libc::getchar(); }
                        // do step back
                        if c == 0 {
                            println!("searching is done");
                            break;
                        }
                        c -= 1;
                        let (ti,tx,ty) = stack[c];
                        i = ti + 1;
                        x = tx;
                        y = ty;
                        f[x as usize][y as usize] = 0;
                        //println!("step back");
                    }
                } else {
                    // next move
                    i += 1;
                    //println!("X");
                }
            } else {
                if c > maxc {
                    maxc = c;
                    println!("{}", maxc);
                }
                // do step back
                if c == 0 {
                    println!("searching is done");
                    break;
                }
                c -= 1;
                let (ti,tx,ty) = stack[c];
                i = ti + 1;
                x = tx;
                y = ty;
                f[x as usize][y as usize] = 0;
                //println!("step back");
            }
            //unsafe { libc::getchar(); }
        //}
        //println!("{:?}", time::Instant::now() - start);
    }
}
