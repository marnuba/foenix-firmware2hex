# foenix-firmware2hex

This is a small tool to convert firmware releases for the Foenix F256k home computer into hex files suitable for FoenixIDE.

If you don't know what any of this means, you probably won't need this tool.

Usage:
Just place the tool in the same folder as the bulk.csv (or somewhere in the PATH) and run it in that very folder.
Currently there are no command line arguments, the output file name is always _kernel_F256jr.hex_.

Official firmware release can be found on the [Firmware Release Page](https://github.com/FoenixRetro/f256-firmware/releases).

Feel free to edit the bulk.csv to add or remove content in the reulting hex file.
