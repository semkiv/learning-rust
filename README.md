# learning-rust

A toy repository for keeping track of and syncing the progress of learning the Rust programming language.

Based on [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html).

Below are some useful config files for building, running and debugging in VS Code.

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
                "command": "cargo",
                "args": [
                    "check"
                ],
                "problemMatcher": [],
                "group": "none"
            },
            {
                "label": "Build Dev",
                "type": "shell",
                "command": "cargo",
                "args": [
                    "build",
                ],
                "problemMatcher": [],
                "group": {
                    "kind": "build",
                    "isDefault": true
                }
            },
            {
                "label": "Build Release",
                "type": "shell",
                "command": "cargo",
                "args": [
                    "build",
                    "--release"
                ],
                "problemMatcher": [],
                "group": "build"
            },
            {
                "label": "Test",
                "type": "shell",
                "command": "cargo",
                "args": [
                    "test"
                ],
                "problemMatcher": [],
                "group": {
                    "kind": "test",
                    "isDefault": true
                }
            },
            {
                "label": "Build Documentation",
                "type": "shell",
                "command": "cargo",
                "args": [
                    "doc"
                ],
                "problemMatcher": [],
                "group": "none"
            },
            {
                "label": "Build Documentation & Open",
                "type": "shell",
                "command": "cargo",
                "args": [
                    "doc",
                    "--open"
                ],
                "linux": {
                    // See this: https://stackoverflow.com/a/66210360
                    "command": "setsid",
                    "args": [
                        "cargo",
                        "doc",
                        "--open"
                    ],
                },
                "problemMatcher": [],
                "group": "none"
            }
        ]
    }
    ```

* **`launch.json`**

    ```jsonc
    {
        // Use IntelliSense to learn about possible attributes.
        // Hover to view descriptions of existing attributes.
        // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
        "version": "0.2.0",
        "configurations": [
            {
                "name": "Launch",
                "type": "cppdbg",
                "request": "launch",
                "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}",
                "args": [],
                "stopAtEntry": false,
                "cwd": "${workspaceFolder}",
                "environment": [],
                "externalConsole": false,
                "linux": {
                    "miDebuggerPath": "rust-gdb",
                    "MIMode": "gdb",
                    "setupCommands": [
                        {
                            "description": "Enable pretty-printing for gdb",
                            "text": "-enable-pretty-printing",
                            "ignoreFailures": true
                        }
                    ]
                },
                "windows": {
                    "type": "cppvsdbg",
                    "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}.exe",
                    "console": "externalTerminal"
                }
            },
            {
                "name": "Build (Dev) & Launch",
                "type": "cppdbg",
                "preLaunchTask": "Build Dev",
                "request": "launch",
                "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}",
                "args": [],
                "stopAtEntry": false,
                "cwd": "${workspaceFolder}",
                "environment": [],
                "externalConsole": false,
                "linux": {
                    "miDebuggerPath": "rust-gdb",
                    "MIMode": "gdb",
                    "setupCommands": [
                        {
                            "description": "Enable pretty-printing for gdb",
                            "text": "-enable-pretty-printing",
                            "ignoreFailures": true
                        }
                    ]
                },
                "windows": {
                    "type": "cppvsdbg",
                    "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}.exe",
                    "console": "externalTerminal"
                }
            },
            {
                "name": "Build (Release) & Launch",
                "type": "cppdbg",
                "preLaunchTask": "Build Release",
                "request": "launch",
                "program": "${workspaceFolder}/target/release/${workspaceFolderBasename}",
                "args": [],
                "stopAtEntry": false,
                "cwd": "${workspaceFolder}",
                "environment": [],
                "externalConsole": false,
                "linux": {
                    "miDebuggerPath": "rust-gdb",
                    "MIMode": "gdb",
                    "setupCommands": [
                        {
                            "description": "Enable pretty-printing for gdb",
                            "text": "-enable-pretty-printing",
                            "ignoreFailures": true
                        }
                    ]
                },
                "windows": {
                    "type": "cppvsdbg",
                    "program": "${workspaceFolder}/target/release/${workspaceFolderBasename}.exe",
                    "console": "externalTerminal"
                }
            }
        ]
    }
    ```
