#include <assert.h>
#include <stdlib.h>
#include <string.h>

#include "c_lib.h"

#define RET "Hello From C!"

int c_lib_hello(char** str) {
    assert(str != NULL);
    *str = malloc(sizeof(RET) + 1);
    if (!*str) {
        return -1;
    }
    strcpy(*str, RET);
    return 0;
}
