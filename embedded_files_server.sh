#!/bin/bash

SAVE_PWD=$(pwd)
SCRIPT_RELATIVE_DIR=$(dirname "${BASH_SOURCE[0]}")
cd $SCRIPT_RELATIVE_DIR

touch src/main.rs

if [ -z "$1" ]
then
	echo "No path specified, using default test_folder"
	EMBEDDED_FILES_PATH=test_folder cargo build --release
else
	EMBEDDED_FILES_PATH=$1 cargo build --release
fi

cd $SAVE_PWD