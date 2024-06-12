#include <QMainWindow>
#include <QWidget>

#include "ui_mainwindow.h"

class MainWindow : public QMainWindow {
    Q_OBJECT

  public:
    MainWindow(QWidget* parent = nullptr)
        : QMainWindow(parent), ui(new Ui::MainWindow) {
        ui->setupUi(this);
    }
    ~MainWindow() { delete ui; }

  private:
    Ui::MainWindow* ui;
};
