#include <stdio.h>
#include <stdint.h>
#include "my_rust_lib_binding.h"

#define PROCESSING_COUNT  10

int main() {
    // Test simple
    const uint32_t a = 7;
    const uint32_t b = 5;
    const uint32_t result = do_add(a, b);
    printf("Adding %u + %u result = %u\n", a, b, result);

    Processor* proc = processor_create();
    printf("Enque %d processings...\n", PROCESSING_COUNT);

    for (uint8_t i = 0; i < PROCESSING_COUNT; ++i) {
        processor_enque_add(proc, 1, ((int32_t)i) + 10);
    }

    printf("Enqued 3 processings! Awaiting results...\n");

    for (uint8_t i = 0; i < PROCESSING_COUNT; ++i) {
        int32_t result_value;
        const int32_t result_expected = 1 + ((int32_t)i) + 10;

        const ProcessingStatus status = processor_poll_result(proc, &result_value, 1000);
        if (status == Ok) {
            printf("Got %u result! Is result %d == %d expected?\n", i, result_value, result_expected);
        } else {
            printf("Getting %u result failed, reason = %d. \n", i, (int32_t)status);
        }
    }

    processor_free(proc);
    printf("Processor freed, should drop be triggered soon...\n");

    return 0;
}