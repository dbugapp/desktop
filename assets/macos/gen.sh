#!/bin/bash

sips -z 16 16     icon.png --out dbug.iconset/icon_16x16.png
sips -z 32 32     icon.png --out dbug.iconset/icon_16x16@2x.png
sips -z 32 32     icon.png --out dbug.iconset/icon_32x32.png
sips -z 64 64     icon.png --out dbug.iconset/icon_32x32@2x.png
sips -z 128 128   icon.png --out dbug.iconset/icon_128x128.png
sips -z 256 256   icon.png --out dbug.iconset/icon_128x128@2x.png
sips -z 256 256   icon.png --out dbug.iconset/icon_256x256.png
sips -z 512 512   icon.png --out dbug.iconset/icon_256x256@2x.png
sips -z 512 512   icon.png --out dbug.iconset/icon_512x512.png
cp icon.png              dbug.iconset/icon_512x512@2x.png

iconutil -c icns dbug.iconset

mv dbug.icns ./Dbug.app/Contents/Resources/dbug.icns
