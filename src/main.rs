extern crate libc;

fn print_array(arr: &[[usize; 10]; 10]) {
    for y in 0..10 {
        for x in 0..10 {
            print!("{:2} ", arr[x][y]);
        }
        println!();
    }
}

fn main() {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut c: isize = 0;
    let mut maxc: isize = 0;
    let mut f: [[usize; 10]; 10] = [[0; 10]; 10];
    let moves: [(isize,isize); 8] = [(1,2), (2,1), (2,-1), (1,-2), (-2,-1), (-2,1), (-1,-2), (-1,2)];
    let mut i = 0;
    let mut stack: [(usize,isize,isize); 101] = [(0,0,0); 101];

    loop {
        //use std::time;
        //let start = time::Instant::now();
        //for _ in 0..1_000_000_000 {
            if i < moves.len() {
                //print!("{}: try move {}: ", c, i);
                let (dx,dy) = moves[i];
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
                c -= 1;
                if c < 0 {
                    println!("searching is done");
                    break;
                }
                let (ti,tx,ty) = stack[c as usize];
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
