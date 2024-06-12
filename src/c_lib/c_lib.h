#ifndef C_LIB_H
#define C_LIB_H

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

typedef enum c_lib_err { C_LIB_SUCCESS, C_LIB_ERROR } c_lib_err;

c_lib_err c_lib_func(char** str);

#ifdef __cplusplus
}
#endif // __cplusplus
#endif // !C_LIB_H
