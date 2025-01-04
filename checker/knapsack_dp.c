#include <stdint.h>
#include <stdio.h>

int knap_sack(uint64_t W, uint32_t wt[], uint32_t val[], uint32_t n) {
    uint32_t dp[W + 1];

    for (size_t i = 0; i <= W; i++) {
        dp[i] = 0;
    }

    for (size_t i = 0; i < n; i++) {
        for (size_t j = W; j >= wt[i]; j--) {
            dp[j] = dp[j] > dp[j - wt[i]] + val[i] ? dp[j] : dp[j - wt[i]] + val[i];
        }
    }

    return dp[W];
}

int main() {
    uint32_t n;
    uint64_t w;
    scanf("%u %lu", &n, &w);

    if (n == 0 || w == 0) {
        printf("0\n");
        return 0;
    }

    uint32_t weights[n], values[n];
    for (size_t i = 0; i < n; i++) {
        scanf("%u %u", &values[i], &weights[i]);
    }

    printf("%d\n", knap_sack(w, weights, values, n));
    return 0;
}
