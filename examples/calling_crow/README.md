# Example 2:

Creating a multivariant using CROW to generate the variants. The source code `f.c` is passed to CROW which generates a handful number of semantically equivalent variants. The generated variants are passed to our linker and the multivariant library is created. The final step finish the compilation to an executable binary. The scripting can be found at `build_multivariant.sh`.