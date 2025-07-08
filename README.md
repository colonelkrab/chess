The main goal of this project was to learn rust and finish it without using any libraries other than macroquad. Another goal was to come up with an implementation of Chess on my own. Therefore, I did not look up the standard ways to implement it. Ofc my implementation ended up being messy and lengthy. There are lot of optimizations that can be done in many places and also a lot of code to clean up. I might revisit this project in the future to do those things, but for now its done.

### run 
- make sure the built binary has access to assets folder in the same directory.
- webGL needs to be enabled to run the browser version

### nixos
macroquad won't run without certain paths set in LD. I used the following shell setup and found success running the binary.
```
# shell.nix
{pkgs ? import <nixpkgs> {}}: let
  libPath = pkgs.lib.makeLibraryPath [
    pkgs.xorg.libX11
    pkgs.libxkbcommon
    pkgs.xorg.libXi
    pkgs.libGL
  ];
in
  pkgs.mkShell {
    nativeBuildInputs = with pkgs.buildPackages; [
      cargo
      rustc
    ];
    LD_LIBRARY_PATH = libPath;
  }
```
