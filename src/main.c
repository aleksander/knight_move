#include <stdio.h>
#include <time.h>
#include <stddef.h>

void print_array (int arr[10][10]) {
    for (int y=0; y<10; ++y) {
        for (int x=0; x<10; ++x) {
            printf("%2u ", arr[x][y]);
        }
        printf("\n");
    }
}

int main() {
    int x = 0;
    int y = 0;
    int c = 0;
    //TODO use plain array: f[100]
    int f[10][10] = { 0 };
    int moves[8][2] = {
        {1,2}, {2,1}, {2,-1}, {1,-2}, 
        {-2,-1}, {-2,1}, {-1,-2}, {-1,2}
    };
    int i = 0;
    int stack[101][3] = { 0 };

    for (;;) {
        struct timespec start;
        clock_gettime(CLOCK_MONOTONIC, &start);
        for (int o=0; o<1000000000; ++o) {
            if (i < 8) {
                //print!("{}: try move {}: ", c, i);
                int nx = x + moves[i][0];
                int ny = y + moves[i][1];
                if ((nx >= 0) && (nx <= 9) && (ny >= 0) && (ny <= 9) && (f[nx][ny] == 0)) {
                    // save current position to stack
                    stack[c][0] = i;
                    stack[c][1] = x;
                    stack[c][2] = y;
                    // save current position to field
                    f[x][y] = ++c;
                    // do move
                    //println!("move ({},{}) -> ({},{})", x, y, nx, ny);
                    //print_array(&f);
                    x = nx;
                    y = ny;
                    if (c == 99) {
                        printf("solution is found!\n");
                        print_array(f);
                    }
                    // reset move counter
                    i = 0;
                } else {
                    // next move
                    ++i;
                    //println!("X");
                }
            } else {
                // do step back
                if (c == 0) {
                    printf("searching is done\n");
                    break;
                }
                --c;
                i = stack[c][0];
                x = stack[c][1];
                y = stack[c][2];
                ++i;
                f[x][y] = 0;
                //println!("step back");
            }
            //unsafe { libc::getchar(); }
        }
        struct timespec stop;
        clock_gettime(CLOCK_MONOTONIC, &stop);
        size_t sec = stop.tv_sec-start.tv_sec;
        size_t nsec = sec * 1000000000;
        nsec += stop.tv_nsec;
        nsec -= start.tv_nsec;
        printf("%lu\n", nsec);
    }
}
