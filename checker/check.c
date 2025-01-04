#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char* argv[]) {
    // Get output form argv
    uint64_t best_value = atoi(argv[1]);
    size_t   solution[argc - 2];
    for (int i = 2; i < argc; i++) {
        solution[i - 2] = atoi(argv[i]);
    }

    // Read input file (will be redirected)

    uint32_t n;
    uint64_t w;
    scanf("%u %lu", &n, &w);
    uint32_t weights[n], values[n];
    for (size_t i = 0; i < n; i++) {
        scanf("%u %u", &values[i], &weights[i]);
    }

    uint64_t total_weight = 0;
    uint64_t total_value  = 0;

    for (int i = 0; i < argc - 2; i++) {
        total_weight += (uint64_t)weights[solution[i]];
        total_value += (uint64_t)values[solution[i]];
    }

    if (total_weight > w) {
        printf("Weight limit exceeded: %lu\n", total_weight);
        return 1;
    }

    if (total_value != best_value) {
        printf("Incorrect value\n");
        return 1;
    }

    return 0;
}

