#ifndef MY_RUST_LIB_BINDING_H
#define MY_RUST_LIB_BINDING_H

#include <stdint.h>

typedef enum {
    Ok,
    Timeout,
    Overflow,
    EnqueFailed,
    OtherError,
} ProcessingStatus;

typedef struct Processor Processor;

Processor* processor_create();
void processor_free(Processor* processor);

ProcessingStatus processor_enque_add(Processor* processor, int32_t left, int32_t right);
ProcessingStatus processor_enque_sub(Processor* processor, int32_t left, int32_t right);
ProcessingStatus processor_poll_result(Processor* processor, int32_t* result_value, uint64_t timeout_millis);

uint32_t do_add(uint32_t a, uint32_t b);

#endif