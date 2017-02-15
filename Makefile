all:
	gcc -o knight_move src/main.c -Ofast -Wall -Wextra -march=native -fPIC -LTO
