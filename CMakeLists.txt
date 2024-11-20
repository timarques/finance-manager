cmake_minimum_required(VERSION 3.15)
project(finance_manager LANGUAGES C)

OPTION(BUILD_FLATPAK "Build flatpak package" OFF)
option(BUILD_INSTALLER "Build Windows installer using Inno Setup" OFF)

find_program(CARGO_EXECUTABLE cargo REQUIRED)
find_program(RUSTC_EXECUTABLE rustc REQUIRED)
find_package(PkgConfig)

set(CMAKE_BUILD_TYPE Release)
set(CARGO_MANIFEST_PATH "${CMAKE_SOURCE_DIR}/Cargo.toml")

execute_process(
    COMMAND ${CARGO_EXECUTABLE}
        metadata
        --manifest-path=${CARGO_MANIFEST_PATH}
        --format-version=1
        --no-deps
    OUTPUT_VARIABLE CARGO_METADATA
    OUTPUT_STRIP_TRAILING_WHITESPACE
)

string(JSON APP_VERSION GET ${CARGO_METADATA} "packages" 0 "version")
string(JSON APP_NAME GET ${CARGO_METADATA} "packages" 0 "name")
string(JSON APP_AUTHORS GET ${CARGO_METADATA} "packages" 0 "authors" 0)
string(JSON APP_DESCRIPTION GET ${CARGO_METADATA} "packages" 0 "description")
string(JSON APP_LICENSE GET ${CARGO_METADATA} "packages" 0 "license")
string(JSON APP_HOMEPAGE GET ${CARGO_METADATA} "packages" 0 "homepage")
string(JSON APP_REPOSITORY GET ${CARGO_METADATA} "packages" 0 "repository")

if (WIN32)
    set(APP_EXECUTABLE_EXTENSION ".exe")
    if (CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
        message(WARNING "Using GCC for Windows. This may not work as expected.")
    endif()
endif()

set(APP_OUTPUT ${CMAKE_BINARY_DIR})
set(APP_ROOT ${CMAKE_SOURCE_DIR})
set(APP_ID com.github.timarques.${APP_NAME})
set(APP_ICON ${APP_ID}.svg)
set(APP_ICON_NAME ${APP_ID})
set(APP_ICON_ICO_PATH ${APP_ROOT}/resources/icon.ico)
set(APP_TITLE "Finance Manager")
set(APP_RESOURCES ${CMAKE_BINARY_DIR}/resources)
set(APP_EXECUTABLE ${APP_NAME}${APP_EXECUTABLE_EXTENSION})
set(APP_G_RESOURCES_XML ${CMAKE_BINARY_DIR}/gresources.xml)
set(APP_G_RESOURCES_ID com/github/timarques/${APP_NAME})

if (UNIX AND NOT APPLE AND BUILD_FLATPAK)
    configure_file("${CMAKE_SOURCE_DIR}/resources/templates/flatpak.yml.template" ${CMAKE_BINARY_DIR}/${APP_ID}.yml @ONLY)
    message(STATUS "Flatpak manifest generated: ${CMAKE_BINARY_DIR}/${APP_ID}.yml")
    return()
endif()

if(WIN32 AND (BUILD_INSTALLER OR NOT PkgCongig_FOUND))
    set(APP_GTK ${CMAKE_BINARY_DIR}/gtk4)
    set(GVSBUILD_ZIP ${CMAKE_BINARY_DIR}/gvsbuild.zip)
    set(GVSBUILD_URL "https://github.com/wingtk/gvsbuild/releases/download/2024.11.1/GTK4_Gvsbuild_2024.11.1_x64.zip")
    
    if(NOT EXISTS ${APP_GTK})
        message(STATUS "Attempting to download and install gvsbuild...")
        file(DOWNLOAD ${GVSBUILD_URL} ${GVSBUILD_ZIP} SHOW_PROGRESS
            STATUS DOWNLOAD_STATUS
            LOG DOWNLOAD_LOG)
            
        list(GET DOWNLOAD_STATUS 0 STATUS_CODE)
        if(NOT STATUS_CODE EQUAL 0)
            message(FATAL_ERROR "Failed to download GTK4: ${DOWNLOAD_LOG}")
        endif()
        
        file(MAKE_DIRECTORY ${APP_GTK})
        execute_process(
            COMMAND ${CMAKE_COMMAND} -E tar xzf ${GVSBUILD_ZIP}
            WORKING_DIRECTORY ${APP_GTK}
            RESULT_VARIABLE EXTRACT_RESULT
        )
        
        if(NOT EXTRACT_RESULT EQUAL 0)
            message(FATAL_ERROR "Failed to extract GTK4 archive")
        endif()

        file(REMOVE ${GVSBUILD_ZIP})
    endif()

    set(APP_PATH "${APP_GTK}\\bin;")
    set(PKG_CONFIG_PATH "${APP_GTK}\\lib\\pkgconfig;")
elseif(PkgConfig_FOUND)
    pkg_check_modules(GTK4 REQUIRED gtk4>=4.12)
    pkg_check_modules(LIBADWAITA REQUIRED libadwaita-1>=1.6)
    pkg_check_modules(GLIB REQUIRED glib-2.0)
    pkg_check_modules(GIO REQUIRED gio-2.0)

    set(APP_PATH $ENV{PATH})
    set(PKG_CONFIG_PATH $ENV{PKG_CONFIG_PATH})
else()
    message(FATAL_ERROR "Could not find GTK4 or PkgConfig")
endif()

configure_file("${CMAKE_SOURCE_DIR}/resources/templates/gresources.xml.template" ${CMAKE_BINARY_DIR}/gresources.xml @ONLY)
configure_file("${CMAKE_SOURCE_DIR}/resources/icon.svg" ${CMAKE_BINARY_DIR}/resources/${APP_ICON} COPYONLY)
file(COPY "${CMAKE_SOURCE_DIR}/resources/styles.css" DESTINATION ${CMAKE_BINARY_DIR}/resources)
file(COPY "${CMAKE_SOURCE_DIR}/resources/icons" DESTINATION ${CMAKE_BINARY_DIR}/resources)

add_custom_target(cargo_build ALL
    COMMAND ${CMAKE_COMMAND} 
        -E env 
            PATH=${APP_PATH}
            PKG_CONFIG_PATH=${PKG_CONFIG_PATH}
            APP_ID=${APP_ID}
            APP_TITLE=${APP_TITLE}
            APP_ICON_NAME=${APP_ICON_NAME}
            APP_ICON_ICO_PATH=${APP_ICON_ICO_PATH}
            APP_RESOURCES=${APP_RESOURCES}
            APP_G_RESOURCES_ID=${APP_G_RESOURCES_ID}
            APP_G_RESOURCES_XML=${APP_G_RESOURCES_XML}
        ${CARGO_EXECUTABLE}
            install
            --path .
            --force
            --color=always
            --target-dir=${CMAKE_BINARY_DIR}
            --root=${APP_OUTPUT}
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    USES_TERMINAL
)

if(WIN32 AND BUILD_INSTALLER)
    find_program(ISCC_PATH 
        NAMES "ISCC.exe" 
        PATHS 
            "$ENV{ProgramFiles\(x86\)}/Inno Setup 6"
            "$ENV{ProgramFiles}/Inno Setup 6"
            "$ENV{ProgramFiles\(x86\)}/Inno Setup 5"
            "$ENV{ProgramFiles}/Inno Setup 5"
        DOC "Path to Inno Setup Compiler"
    )

    if(NOT ISCC_PATH)
        message(FATAL_ERROR "Inno Setup Compiler (ISCC.exe) not found. Please install Inno Setup or disable BUILD_INSTALLER option.")
    endif()

    set(APP_ICON ${APP_ID}.ico)
    file(TO_NATIVE_PATH ${APP_RESOURCES} APP_RESOURCES)
    file(TO_NATIVE_PATH ${APP_ROOT} APP_ROOT)
    file(TO_NATIVE_PATH ${APP_OUTPUT} APP_OUTPUT)
    file(TO_NATIVE_PATH ${APP_GTK} APP_GTK)

    configure_file(${APP_ICON_ICO_PATH} ${CMAKE_BINARY_DIR}/resources/${APP_ICON} COPYONLY)
    configure_file(${CMAKE_SOURCE_DIR}/resources/templates/installer.iss.template ${CMAKE_BINARY_DIR}/installer.iss @ONLY)

    add_custom_target(build_installer ALL
        COMMAND ${CMAKE_COMMAND} -E echo "Building Windows installer..."
        COMMAND ${ISCC_PATH} /O"${CMAKE_BINARY_DIR}/bin" /F"${APP_NAME}-installer" ${CMAKE_BINARY_DIR}/installer.iss
        COMMAND ${CMAKE_COMMAND} -E echo "Windows installer built: ${CMAKE_BINARY_DIR}/bin/${APP_NAME}-installer.exe"
        WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
        USES_TERMINAL
    )

    add_dependencies(build_installer cargo_build)

endif()

if(UNIX AND NOT APPLE)
    include(GNUInstallDirs)

    configure_file("${CMAKE_SOURCE_DIR}/resources/templates/app.desktop.template" ${CMAKE_BINARY_DIR}/${APP_ID}.desktop @ONLY)
    configure_file("${CMAKE_SOURCE_DIR}/resources/templates/metainfo.xml.template" ${CMAKE_BINARY_DIR}/${APP_ID}.metainfo.xml @ONLY)

    install(PROGRAMS ${CMAKE_BINARY_DIR}/bin/${APP_EXECUTABLE} DESTINATION ${CMAKE_INSTALL_BINDIR})
    install(FILES ${CMAKE_BINARY_DIR}/${APP_ID}.desktop DESTINATION ${CMAKE_INSTALL_DATADIR}/applications)
    install(FILES ${CMAKE_BINARY_DIR}/resources/${APP_ICON} DESTINATION ${CMAKE_INSTALL_DATADIR}/icons/hicolor/scalable/apps)
    install(FILES ${CMAKE_BINARY_DIR}/${APP_ID}.metainfo.xml DESTINATION share/metainfo)
endif()