@echo off

title NFT build


cargo build --all --target wasm32-unknown-unknown --release
xcopy %CD%\target\wasm32-unknown-unknown\release\*.wasm %CD%\out\main.wasm* /Y 
echo "el archivo se encuentra ./out/main.wasm "
near dev-deploy
