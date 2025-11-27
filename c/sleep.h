#include<time.h>

void sleep(unsigned int x) {
	clock_t c1 = clock(), c2;
	do {
		if ((c2 = clock()) == (clock_t)-1) {
			break;
		}
	} while (1000.0 * (c2 - c1) / CLOCKS_PER_SEC < x);
}
