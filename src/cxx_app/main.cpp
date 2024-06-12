#include <iostream>

#include <QApplication>

#include "c_lib.h"
#include "cxx_lib.hpp"
#include "mainwindow.hpp"
#include "rs_lib.h"

int main(int argc, char* argv[]) {
    std::cout << cxx_lib::func() << '\n';
    char* pbuf;
    c_lib_func(&pbuf);
    std::cout << pbuf << '\n';
    free(pbuf);

    char* rbuf = rs_lib_func();
    std::cout << rbuf << '\n';
    free(rbuf);

    QApplication app(argc, argv);
    MainWindow mw;

    mw.show();

    return app.exec();
}
