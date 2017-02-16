extern crate libc;

fn print_array (arr: &[[usize; 10]; 10]) {
    for y in 0..10 {
        for x in 0..10 {
            print!("{:2} ", arr[x][y]);
        }
        println!();
    }
}

/*
fn have_unreachable_points (arr: &[[usize; 10]; 10], neighbors: &[[usize; 10]; 10]) -> bool {
    let mut single_neighbor_point = 0;
    for y in 0..10 {
        for x in 0..10 {
            if arr[x][y] == 0 {
                // . . . . . . .
                // . . x . x . .
                // . x . . . x .
                // . . . o . . .
                // . x . . . x .
                // . . x . x . .
                // . . . . . . .
                //
                // o is unreachable
                // FIXME o is unreachable only if there is no last c in neighbors
                if neighbors[x][y] == 0 {
                    return true;
                } /*else if neighbors[x][y] == 1 {
                    //FIXME do not increase if one of the neighbors is our last move
                    //      because in that case there is two free neighbors
                    single_neighbor_point += 1;
                    if single_neighbor_point > 2 {
                        return true;
                    }
                }*/
            }
        }
    }
    false
}
*/

#[inline(always)]
fn have_unreachable_points2 (arr: &[[usize; 10]; 10], neighbors_count: &[[usize; 10]; 10], neighbors: &[[Vec<(usize,usize)>; 10]; 10], last_move: (usize,usize)) -> bool {
    for y in 0..10 {
        'next_x: for x in 0..10 {
            if arr[x][y] == 0 {
                if neighbors_count[x][y] == 0 {
                    for &point in neighbors[x][y].iter() {
                        if point == last_move { continue 'next_x; }
                    }
                    return true;
                }
            }
        }
    }
    false
}

const MOVES: [(isize,isize); 8] = [(1,2), (2,1), (2,-1), (1,-2), (-2,-1), (-2,1), (-1,-2), (-1,2)];

fn main() {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut c: usize = 0;
    //let mut maxc: usize = 0;
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
            //print!("{} ", neighbors_count[x][y]);
        }
        //println!();
    }
    let mut i = 0;
    let mut stack: [(usize,usize,usize); 101] = [(0,0,0); 101];

    f[x][y] = c + 1;
    for &(nx,ny) in neighbors[x][y].iter() {
        neighbors_count[nx][ny] -= 1;
    }

    loop {
        use std::time;
        let start = time::Instant::now();
        for _ in 0..1_000_000_000 {
            if i < neighbors[x][y].len() {
                //print!("{}: try move {}: ", c, i);
                let (nx,ny) = neighbors[x][y][i];
                if f[nx][ny] == 0 {
                    // save current position to stack
                    stack[c] = (i,x,y);
                    // do move
                    c += 1;
                    //println!("move ({},{}) -> ({},{})", x, y, nx, ny);
                    x = nx;
                    y = ny;
                    // save new position to field
                    f[x][y] = c + 1;
                    for &(nx,ny) in neighbors[x][y].iter() {
                        neighbors_count[nx][ny] -= 1;
                    }
                    //print_array(&f);
                    if c == 99 {
                        println!("solution is found!");
                        print_array(&f);
                    }
                    // reset move counter
                    i = 0;
                    if have_unreachable_points2(&f, &neighbors_count, &neighbors, (x,y)) {
                        //println!("{}: isolated points!", c);
                        //print_array(&f);
                        //unsafe { libc::getchar(); }
                        // do step back
                        if c == 0 {
                            println!("searching is done");
                            break;
                        }
                        f[x][y] = 0;
                        for &(nx,ny) in neighbors[x][y].iter() {
                            neighbors_count[nx][ny] += 1;
                        }
                        c -= 1;
                        i = stack[c].0 + 1;
                        x = stack[c].1;
                        y = stack[c].2;
                        //println!("step back");
                    }
                } else {
                    // next move
                    i += 1;
                    //println!("X");
                }
            } else {
                //if c > maxc {
                //    maxc = c;
                //    println!("{}", maxc);
                //}
                // do step back
                if c == 0 {
                    println!("searching is done");
                    break;
                }
                f[x][y] = 0;
                for &(nx,ny) in neighbors[x][y].iter() {
                    neighbors_count[nx][ny] += 1;
                }
                c -= 1;
                i = stack[c].0 + 1;
                x = stack[c].1;
                y = stack[c].2;
                //println!("step back");
            }
            //unsafe { libc::getchar(); }
        }
        println!("{:?}", time::Instant::now() - start);
    }
}
