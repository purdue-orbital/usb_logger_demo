{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell{
	buildInputs = with pkgs; [
		gcc
		rustup

		iw

		pkg-config
		fontconfig

        libxkbcommon
        libGL

        wayland

        xorg.libXcursor
        xorg.libXrandr
        xorg.libXi
        xorg.libX11
        xorg.libxcb

        freetype

        git
        cmake

        pcapc
        libpcap
	];
	LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath [
        pkgs.libxkbcommon
        pkgs.libGL
        pkgs.fontconfig

        pkgs.wayland

        pkgs.xorg.libXcursor
        pkgs.xorg.libXrandr
        pkgs.xorg.libXi
        pkgs.xorg.libX11
        pkgs.xorg.libxcb
        pkgs.freetype

        pkgs.pcapc
        pkgs.libpcap
    ]}";
}
