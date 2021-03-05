# learning-rust

A toy repository for keeping track of and syncing the progress of learning the Rust programming language.

Based on [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html).

Below are some useful config files for building, running and debugging in VS Code. `launch.json` differs between Linux and Windows due to different debuggers on those platforms.

* **`tasks.json`**

    ```jsonc
    {
        // See https://go.microsoft.com/fwlink/?LinkId=733558
        // for the documentation about the tasks.json format
        "version": "2.0.0",
        "tasks": [
            {
                "label": "Check",
                "type": "shell",
                "command": "cargo check",
                "problemMatcher": [],
                "group": "none"
            },
            {
                "label": "Build Debug",
                "type": "shell",
                "command": "cargo build",
                "problemMatcher": [],
                "group": {
                    "kind": "build",
                    "isDefault": true
                }
            },
            {
                "label": "Build Release",
                "type": "shell",
                "command": "cargo build --release",
                "problemMatcher": [],
                "group": "build"
            },
            {
                "label": "Test",
                "type": "shell",
                "command": "cargo test",
                "problemMatcher": [],
                "group": {
                    "kind": "test",
                    "isDefault": true
                }
            }
        ]
    }
    ```

* **`launch.json` (GDB on Linux)**

    ```jsonc
    {
        // Use IntelliSense to learn about possible attributes.
        // Hover to view descriptions of existing attributes.
        // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
        "version": "0.2.0",
        "configurations": [
            {
                "name": "(gdb) Launch",
                "type": "cppdbg",
                "request": "launch",
                "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}",
                "args": [],
                "stopAtEntry": false,
                "cwd": "${workspaceFolder}",
                "environment": [],
                "externalConsole": false,
                "MIMode": "gdb",
                "setupCommands": [
                    {
                        "description": "Enable pretty-printing for gdb",
                        "text": "-enable-pretty-printing",
                        "ignoreFailures": true
                    }
                ]
            },
            {
                "name": "(gdb) Build (Debug) & Launch",
                "type": "cppdbg",
                "preLaunchTask": "Build Debug",
                "request": "launch",
                "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}",
                "args": [],
                "stopAtEntry": false,
                "cwd": "${workspaceFolder}",
                "environment": [],
                "externalConsole": false,
                "MIMode": "gdb",
                "setupCommands": [
                    {
                        "description": "Enable pretty-printing for gdb",
                        "text": "-enable-pretty-printing",
                        "ignoreFailures": true
                    }
                ]
            },
            {
                "name": "(gdb) Build (Release) & Launch",
                "type": "cppdbg",
                "preLaunchTask": "Build Release",
                "request": "launch",
                "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}",
                "args": [],
                "stopAtEntry": false,
                "cwd": "${workspaceFolder}",
                "environment": [],
                "externalConsole": false,
                "MIMode": "gdb",
                "setupCommands": [
                    {
                        "description": "Enable pretty-printing for gdb",
                        "text": "-enable-pretty-printing",
                        "ignoreFailures": true
                    }
                ]
            }
        ]
    }
    ```

* **`launch.json` (Visual C++ on Windows)**

    ```jsonc
    {
        // Use IntelliSense to learn about possible attributes.
        // Hover to view descriptions of existing attributes.
        // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
        "version": "0.2.0",
        "configurations": [
            {
                "name": "(Windows) Launch",
                "type": "cppvsdbg",
                "request": "launch",
                "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}.exe",
                "args": [],
                "stopAtEntry": false,
                "cwd": "${workspaceFolder}",
                "environment": [],
                "console": "externalTerminal"
            },
            {
                "name": "(Windows) Build (Debug) & Launch",
                "type": "cppvsdbg",
                "preLaunchTask": "Build Debug",
                "request": "launch",
                "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}.exe",
                "args": [],
                "stopAtEntry": false,
                "cwd": "${workspaceFolder}",
                "environment": [],
                "console": "externalTerminal"
            },
            {
                "name": "(Windows) Build (Release) & Launch",
                "type": "cppvsdbg",
                "preLaunchTask": "Build Release",
                "request": "launch",
                "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}.exe",
                "args": [],
                "stopAtEntry": false,
                "cwd": "${workspaceFolder}",
                "environment": [],
                "console": "externalTerminal"
            }
        ]
    }
    ```
