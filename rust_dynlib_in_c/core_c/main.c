#include <stdio.h>
#include <stdint.h>
#include "my_rust_lib_binding.h"

int main() {
    uint32_t result = do_add(7, 5);
    printf("Result: %u\n", result);
    return 0;
}