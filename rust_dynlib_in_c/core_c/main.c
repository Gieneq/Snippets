#include <stdio.h>
#include <stdint.h>
#include "my_rust_lib_binding.h"

int main() {
    // Test simple
    const uint32_t a = 7;
    const uint32_t b = 5;
    const uint32_t result = do_add(a, b);
    printf("Adding %u + %u result = %u\n", a, b, result);

    return 0;
}