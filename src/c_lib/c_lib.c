#include <assert.h>
#include <stdlib.h>
#include <string.h>

#include "c_lib.h"

#define RET "Hello From C!"

c_lib_err c_lib_func(char** str) {
    assert(str != NULL);
    *str = malloc(sizeof(RET) + 1);
    if (!*str) {
        return C_LIB_ERROR;
    }
    strcpy(*str, RET);
    return C_LIB_SUCCESS;
}
