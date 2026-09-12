# Running

After compiling your source with `komorei package`, which generates the `package.krx` file, you have
a few options for running.

## Android Device / Emulator

Since Komorei is an Android app, you can clone the [Komorei app repository]() and build it with
Android Studio, then run it on a device or emulator. Sources can be transferred directly to the
device and opened in the app to install.

## Source Lists

You can serve a source list on your local network with `komorei serve package.krx`. This will give
you a `localhost` URL that you can add as a source list in Komorei on any device connected to the
same network. Your source can then be installed from the "add source" view inside Komorei, and will
show as an update if you increment the source version number and recompile. Note that you may need
to pull down to refresh the source lists on the browse tab.