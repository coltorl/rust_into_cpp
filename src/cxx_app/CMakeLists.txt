find_package(Qt6 REQUIRED COMPONENTS Widgets)
qt_standard_project_setup()

qt_add_executable(cxx_app
    main.cpp

    mainwindow.hpp
    mainwindow.cpp
    mainwindow.ui
)

target_link_libraries(cxx_app PRIVATE cxx_lib c_lib rs_lib Qt6::Widgets)
