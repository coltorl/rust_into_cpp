#include <qpushbutton.h>

#include "mainwindow.hpp"

#include "rs_to_cxx_lib/lib.h"
#include "rs_lib.h"
#include "fmt/format.h"

MainWindow::MainWindow(QWidget* parent)
    : QMainWindow(parent), ui(new Ui::MainWindow) {
    ui->setupUi(this);

    connect(ui->rust_unsafe_hello, &QPushButton::clicked, this, [this](){
        ui->label->setText(rs_lib_unsafe_hello());
    });

    connect(ui->rust_safe_hello, &QPushButton::clicked, this, [this](){
        ui->label->setText(rs_lib_hello().c_str());
    });

    connect(ui->rust_c_hello, &QPushButton::clicked, this, [this](){
        ui->label->setText(rs_lib_c_hello().c_str());
    });

    connect(ui->rust_cxx_hello, &QPushButton::clicked, this, [this](){
        ui->label->setText(rs_lib_cxx_hello().c_str());
    });

    connect(ui->get_quote, &QPushButton::clicked, this, [this](){
        auto quote_result = rs_lib_quote();

        if (quote_result.success){
            ui->label->setText(quote_result.data.h.c_str());
        } else {
            // WARN: unsafe, will crash
            // ui->label->setText(quote_result.data.h.c_str());
            ui->label->setText(fmt::format("<font color='red'>{}", quote_result.msg.c_str()).c_str());
        }
    });

}
