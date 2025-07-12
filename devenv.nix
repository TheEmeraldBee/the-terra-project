{
  pkgs,
  lib,
  ...
}: {
  env.LD_LIBRARY_PATH = with pkgs;
    lib.makeLibraryPath [
      wayland
      alsa-lib
      udev
      xorg.libX11
      xorg.libXcursor
      xorg.libXrandr
      xorg.libXi
      xorg.libxcb
      libGL
      vulkan-loader
      vulkan-headers
      libxkbcommon
    ];

  languages.rust.enable = true;
}
