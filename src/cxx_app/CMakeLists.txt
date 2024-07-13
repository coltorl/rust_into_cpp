qt_standard_project_setup()

qt_add_executable(cxx_app
    main.cpp

    mainwindow.hpp
    mainwindow.cpp
    mainwindow.ui
)

target_link_libraries(cxx_app PRIVATE 
    Qt6::Widgets fmt::fmt
    cxx_lib c_lib rs_lib rs_to_cxx_lib
)

add_custom_target(run_cxx_app
    COMMAND cxx_app >> $ENV{CXX_APP_LOGS} 2>&1
    DEPENDS cxx_app
    WORKING_DIRECTORY ${CMAKE_PROJECT_DIR}
)
